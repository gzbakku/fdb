#[macro_use]
extern crate lazy_static;

use std::thread;
use std::time::Duration;

//extern crate clap;
use clap;
//use clap::{Arg, App};

use std::sync::Mutex;
use std::collections::HashMap;
use std::path::Path;

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
mod common;
mod auth;

#[derive(Debug)]
#[allow(non_camel_case_types)]
struct Actor_Template {
    id:String,
    sig:String
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
struct Session_Template {
    id:String,
    sig:String
}

lazy_static! {
    #[derive(Debug)]
    static ref KEY: Mutex<Vec<String>> =  Mutex::new(vec![]);
    static ref BASE_KEY: Mutex<Vec<String>> =  Mutex::new(vec![]);
    static ref DIR: Mutex<Vec<String>> =  Mutex::new(vec![]);
    static ref SESSION: Mutex<Session_Template> =  Mutex::new(Session_Template {
        id:String::new(),
        sig:String::new()
    });
    static ref ACTOR: Mutex<Actor_Template> =  Mutex::new(Actor_Template {
        id:String::new(),
        sig:String::new()
    });
    static ref BOOK : Mutex<HashMap<String,JsonValue>> = Mutex::new(HashMap::new());
    static ref ACTORS : Mutex<JsonValue> = Mutex::new(JsonValue::new_object());
}

//

//*******************************************************
//main

/*
    cargo run -- --secure=Om2lPq84vgIhsPEhWsh3LdRmNmI2MXpQ --signature=XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb --id=aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5
    --session_id=XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb --session_signature=aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5 --base_dir=d://workstation/expo/rust/fdb/composer/instance --port=8088 --composer=127.0.0.1
*/

fn main(){

    let matches = clap::App::new("Fuc* DB Composer")
                          .version("0.0.1")
                          .author("gzbakku. <gzbakku@gmail.com>")
                          .about("Fuc* DB Fastest NoSql Secure Database Written in Rust")
                           .arg(
                               clap::Arg::with_name("secure")
                                .help("secure encrypting key")
                                .long("secure")
                                .value_name("secure")
                                .required(true)
                            )
                            .arg(
                                clap::Arg::with_name("base_dir")
                                 .help("output base dir path")
                                 .long("base_dir")
                                 .value_name("base_dir")
                                 .required(true)
                             )
                             .arg(
                                 clap::Arg::with_name("id")
                                  .help("actor id")
                                  .long("id")
                                  .value_name("id")
                                  .required(true)
                              )
                              .arg(
                                  clap::Arg::with_name("signature")
                                   .help("actor secure signature")
                                   .short("sig")
                                   .long("signature")
                                   .value_name("signature")
                                   .required(true)
                               )
                               .arg(
                                   clap::Arg::with_name("port")
                                    .help("actor port")
                                    .short("p")
                                    .long("port")
                                    .value_name("port")
                                    .required(true)
                                )
                                .arg(
                                    clap::Arg::with_name("composer")
                                     .help("composer ip adress and port")
                                     .long("composer")
                                     .value_name("composer")
                                     .required(true)
                                 )
                                 .arg(
                                     clap::Arg::with_name("session_id")
                                      .help("session id")
                                      .long("session_id")
                                      .value_name("session_id")
                                      .required(true)
                                  )
                                  .arg(
                                      clap::Arg::with_name("session_signature")
                                       .help("session signature")
                                       .long("session_signature")
                                       .value_name("session_signature")
                                       .required(true)
                                   )
                          .get_matches();

        //------------------------------------
        //extract id

        if matches.is_present("id") {
            match matches.value_of("id") {
                Some(id) => {
                    ACTOR.lock().unwrap().id = id.to_string();
                },
                None => {
                    common::error("not_found-id");
                    return;
                }
            }
        }

        //------------------------------------
        //extract signature

        if matches.is_present("signature") {
            match matches.value_of("signature") {
                Some(signature) => {
                    ACTOR.lock().unwrap().sig = signature.to_string();
                },
                None => {
                    common::error("not_found-signature");
                    return;
                }
            }
        }

        //------------------------------------
        //session id

        if matches.is_present("session_id") {
            match matches.value_of("session_id") {
                Some(id) => {
                    SESSION.lock().unwrap().id = id.to_string();
                },
                None => {
                    common::error("not_found-session_id");
                    return;
                }
            }
        }

        //------------------------------------
        //session signature

        if matches.is_present("session_signature") {
            match matches.value_of("session_signature") {
                Some(signature) => {
                    SESSION.lock().unwrap().sig = signature.to_string();
                },
                None => {
                    common::error("not_found-session_signature");
                    return;
                }
            }
        }

        println!("{:?}",SESSION.lock().unwrap());

        //------------------------------------
        //extract secure

        if matches.is_present("secure") {
            match matches.value_of("secure") {
                Some(secure) => {
                    KEY.lock().unwrap().push(secure.to_string());
                },
                None => {
                    common::error("not_found-secure");
                    return;
                }
            }
        }

        //------------------------------------
        //extract base_dir

        if matches.is_present("base_dir") {
            match matches.value_of("base_dir") {
                Some(path) => {
                    if Path::new(&path).exists() == false {
                        println!("path : {:?}",&path);
                        common::error("invalid-base_dir : path does not exists");
                        return;
                    }
                    match io::make_base_dirs(&path.to_string()) {
                        Ok(_) => {},
                        Err(_) => {
                            common::error("failed-make_base_dirs-initiate-files-fdb");
                            return;
                        }
                    }
                    DIR.lock().unwrap().push(path.to_string());
                },
                None => {
                    common::error("not_found-base_dir");
                    return;
                }
            }
        }

        //------------------------------------
        //set base key

        let session = SESSION.lock().unwrap();
        let actor = ACTOR.lock().unwrap();

        let session_sig_string = format!("{}{}",actor.sig,session.sig);
        let session_hash = common::hash(session_sig_string);

        BASE_KEY.lock().unwrap().push(session_hash.to_string());

        //------------------------------------
        //extract port

        if matches.is_present("port") {
            match matches.value_of("port") {
                Some(port) => {
                    match server(port.to_string()) {
                        Ok(_) => {},
                        Err(_) => {}
                    }
                },
                None => {
                    common::error("not_found-port");
                    return;
                }
            }
        }

}

#[allow(dead_code)]
fn test_main(){

    //let current_dir_object = std::env::current_dir().unwrap();
    //let current_dir = current_dir_object.to_str().unwrap();

    let key = "8cfb30b34977529853bbe46afdbbd5ae".to_string();
    KEY.lock().unwrap().push(key.to_string());

    let dir = "D://workstation/expo/rust/fdb/instance".to_string();
    DIR.lock().unwrap().push(dir);

    let get_dir = DIR.lock().unwrap()[0].to_string();

    match io::make_base_dirs(&get_dir) {
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

        //let connection_info = req.connection_info().clone();

        //let get_dir = DIR.lock().unwrap()[0].to_string();

        let headers = req.headers();
        let base_key = BASE_KEY.lock().unwrap()[0].to_string();

        //let peer = connection_info.remote();
        //println!("peer : {:?}", peer);

        let mut access_granted = true;
        match auth::check(headers,base_key) {
            Ok(_)=>{},
            Err(_) => {
                access_granted = false;
            }
        }

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

    common::success();

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
                web::resource("/kill").route(web::post().to_async(kill))
            )
            .service(
                web::resource("/check").route(web::post().to_async(check))
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

fn check(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|_body| {
        return Ok(server::success());
    })

}

fn end(){
    //panic!("files actor closed");
    thread::spawn(move || {
        // Let's wait 20 milliseconds before notifying the condvar.
        thread::sleep(Duration::from_millis(3000));
        common::log("killing files actor");
        std::process::exit(1);
    });
}

fn kill(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|_body| {
        end();
        return Ok(server::success());
    })

}
