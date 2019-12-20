#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use std::collections::HashMap;

use actix_web::{
    web, App, Error, HttpResponse, HttpServer
};
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceResponse,ServiceRequest};

use futures::{Future, Stream, Poll};
use futures::future::{ok, Either, FutureResult};
use json::JsonValue;

mod server;
mod crypt;
mod io;

lazy_static! {
    #[derive(Debug)]
    static ref KEY: Mutex<Vec<String>> =  Mutex::new(vec![]);
    static ref DIR: Mutex<Vec<String>> =  Mutex::new(vec![]);
    static ref BOOK : Mutex<HashMap<String,JsonValue>> = Mutex::new(HashMap::new());
    static ref CONFIG : Mutex<JsonValue> = Mutex::new(JsonValue::new_object());
}


//*******************************************************
//main

fn main(){

    // let current_dir_object = env::current_dir().unwrap();
    // let current_dir = current_dir_object.to_str().unwrap();

    let key = "8cfb30b34977529853bbe46afdbbd5ae".to_string();
    KEY.lock().unwrap().push(key.to_string());

    let dir = "D://workstation/expo/rust/fdb/instance".to_string();
    DIR.lock().unwrap().push(dir);

    let get_dir = DIR.lock().unwrap()[0].to_string();

    match io::make_base_dirs(get_dir) {
        Ok(_r) => {},
        Err(_e) => {
            panic!("failed-make_base_dirs-initiate-files-fdb");
        }
    }

    match server("8088".to_string()) {
        Ok(_) => {},
        Err(_) => {}
    }

}

//*******************************************************
//authenticate

pub struct Auth;

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckIp<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckIp { service })
    }
}

pub struct CheckIp<S> {
    service: S,
}

impl<S, B> Service for CheckIp<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {

        let connection_info = req.connection_info().clone();

        //let peer = connection_info.remote();
        //println!("peer : {:?}", peer);

        let access_granted = true;

        if access_granted {
            Either::A(self.service.call(req))
        } else {
            Either::B(ok(req.into_response(
                HttpResponse::Forbidden()
                .set_header("forbidden", "true")
                .finish()
                .into_body()
            )))
        }

    }
}

//*******************************************************
//server functions

fn server(port:String) -> std::io::Result<()> {

    let addr = String::from(format!("127.0.0.1:{}",port));
    println!("listening on port {}",&addr);

    HttpServer::new(|| {
        App::new()
            .wrap(Auth)
            .data(web::JsonConfig::default().limit(40096))
            .service(
                web::resource("/write/encrypted").route(web::post().to_async(write_encrypted))
            )
            .service(
                web::resource("/read/encrypted").route(web::post().to_async(read_encrypted))
            )
            .service(
                web::resource("/write/json").route(web::post().to_async(write_file))
            )
            .service(
                web::resource("/read/json").route(web::post().to_async(read_file))
            )
            .service(
                web::resource("/list").route(web::post().to_async(list))
            )
            .service(
                web::resource("/set/key").route(web::post().to_async(set_key))
            )
            .service(
                web::resource("/set/config").route(web::post().to_async(set_config))
            )
    })
    .bind(addr)?
    .run()

}

//*******************************************************
//server routes

fn write_encrypted(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|body| {

        let result = json::parse(std::str::from_utf8(&body).unwrap());
        let injson: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };

        if &injson.has_key("file") == &false {
            return Ok(server::error("not_found-file_name".to_string()));
        }
        if &injson.has_key("data") == &false {
            return Ok(server::error("not_found-file_data".to_string()));
        }
        if &injson["data"].is_object() == &false {
            return Ok(server::error("invalid-file_data".to_string()));
        }

        let opener = &KEY.lock().unwrap();
        let key = &opener[0];

        let data_dump = &injson["data"].dump();
        let encrypted = crypt::encrypt(data_dump.to_string(),key.to_string());
        let combine = format!("{:?};{:?}",encrypted.nonce,encrypted.cipher);
        let get_dir = DIR.lock().unwrap()[0].to_string();

        match io::crypted::write(get_dir,injson["file"].to_string(),combine.as_bytes().to_vec()) {
            Ok(_r) => {},
            Err(e) => {
                return Ok(server::error(e.to_string()));
            }
        }

        let open_book = &mut BOOK.lock().unwrap();
        open_book.insert(injson["file"].to_string(),injson["data"].clone());

        return Ok(server::success());

    })

}

fn read_encrypted(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|body| {

        let result = json::parse(std::str::from_utf8(&body).unwrap());
        let injson: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };

        if &injson.has_key("err") == &true {
            return Ok(server::error("invalid_request-read_file".to_string()));
        }
        if &injson.has_key("file") == &false {
            return Ok(server::error("not_found-read_file-file_name".to_string()));
        }

        //check book
        let file_name = &injson["file"].to_string();
        let open_book = &mut BOOK.lock().unwrap();
        if open_book.contains_key(file_name) {
            return Ok(server::success_with_data(open_book[file_name].clone()));
        }

        //read from file
        let get_dir = DIR.lock().unwrap()[0].to_string();
        let result: io::crypted::CRYPT;
        match io::crypted::read(get_dir,file_name.to_string()) {
            Ok(r) => {
                result = r;
            },
            Err(e) => {
                return Ok(server::error(e.to_string()));
            }
        }

        let opener = &KEY.lock().unwrap();
        let key = &opener[0];

        let decrypted = crypt::decrypt(result.cipher,key.to_string(),result.nonce);
        let result = json::parse(&decrypted);
        let data: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };

        if &data.has_key("err") == &true {
            return Ok(server::error("failed-parse_decrypted_into_object".to_string()));
        }

        open_book.insert(file_name.to_string(),data.clone());

        return Ok(server::success_with_data(data));

    })

}

fn read_file(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|body| {

        let result = json::parse(std::str::from_utf8(&body).unwrap());
        let injson: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };

        if &injson.has_key("err") == &true {
            return Ok(server::error("invalid_request-read_file".to_string()));
        }
        if &injson.has_key("file") == &false {
            return Ok(server::error("not_found-read_file-file_name".to_string()));
        }

        //check book
        let file_name = &injson["file"].to_string();
        let open_book = &mut BOOK.lock().unwrap();
        if open_book.contains_key(file_name) {
            return Ok(server::success_with_data(open_book[file_name].clone()));
        }

        //read from file
        let result: String;
        let get_dir = DIR.lock().unwrap()[0].to_string();
        match io::files::read(get_dir,file_name.to_string()) {
            Ok(s) => {
                result = s;
            },
            Err(_e) => {
                return Ok(server::error("failed-read_file-read_json".to_string()));
            }
        }

        let result = json::parse(&result);
        let data: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };

        if &data.has_key("err") == &true {
            return Ok(server::error("failed-parse_decrypted_into_object".to_string()));
        }

        open_book.insert(file_name.to_string(),data.clone());

        return Ok(server::success_with_data(data));

    })

}

fn write_file(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|body| {

        let result = json::parse(std::str::from_utf8(&body).unwrap());
        let injson: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };

        if &injson.has_key("file") == &false {
            return Ok(server::error("not_found-file_name".to_string()));
        }
        if &injson.has_key("data") == &false {
            return Ok(server::error("not_found-file_data".to_string()));
        }
        if &injson["data"].is_object() == &false {
            return Ok(server::error("invalid-file_data".to_string()));
        }

        let data = injson["data"].dump();
        let get_dir = DIR.lock().unwrap()[0].to_string();

        match io::files::write(get_dir,injson["file"].to_string(),data.as_bytes().to_vec()) {
            Ok(_r) => {},
            Err(e) => {
                return Ok(server::error(e.to_string()));
            }
        }

        let open_book = &mut BOOK.lock().unwrap();
        open_book.insert(injson["file"].to_string(),injson["data"].clone());

        return Ok(server::success());

    })

}

fn list(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|body| {

        let result = json::parse(std::str::from_utf8(&body).unwrap());
        let injson: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };

        if &injson.has_key("location") == &false {
            return Ok(server::error("not_found-file_name-list".to_string()));
        }
        if &injson["location"].is_string() == &false {
            return Ok(server::error("invalid-file_data-list".to_string()));
        }

        let location = &injson["location"].to_string();

        if
            location != &"files".to_string() &&
            location != &"vault".to_string() &&
            location != &"list".to_string()
        {
            return Ok(server::error("invalid-location-list".to_string()));
        }

        let get_dir = DIR.lock().unwrap()[0].to_string();

        let fetch = io::get_files(get_dir,location.to_string());
        match fetch {
            Ok(r) => {
                return Ok(server::success_with_data(r));
            },
            Err(e) => {
                return Ok(server::error(e.to_string()));
            }
        }

    })

}

fn set_key(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|body| {

        let result = json::parse(std::str::from_utf8(&body).unwrap());
        let injson: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };

        if &injson.has_key("key") == &false {
            return Ok(server::error("not_found-file_name-set_key".to_string()));
        }
        if &injson["key"].is_string() == &false {
            return Ok(server::error("invalid-file_data-set_key".to_string()));
        }

        let key = &injson["key"].to_string();

        //let key = "8cfb30b34977529853bbe46afdbbd5ae".to_string();

        KEY.lock().unwrap().push(key.to_string());

        return Ok(server::success());

    })

}

fn set_config(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|body| {

        let result = json::parse(std::str::from_utf8(&body).unwrap());
        let injson: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };

        if &injson.has_key("config") == &false {
            return Ok(server::error("not_found-file_name-set_key".to_string()));
        }
        if &injson["config"].is_object() == &false {
            return Ok(server::error("invalid-file_data-set_key".to_string()));
        }

        CONFIG.lock().unwrap()["config"] = injson["config"].clone();

        return Ok(server::success());

    })

}
