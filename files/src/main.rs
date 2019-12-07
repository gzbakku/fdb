#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use std::collections::HashMap;

use actix_web::{
    web, App, Error, HttpResponse, HttpServer
};

use futures::{Future, Stream};
use json::JsonValue;

mod server;
mod crypt;
mod io;

lazy_static! {
    #[derive(Debug)]
    static ref KEY: Mutex<Vec<String>> =  Mutex::new(vec![]);
    static ref BOOK : Mutex<HashMap<String,JsonValue>> = Mutex::new(HashMap::new());
}

fn main() -> std::io::Result<()> {

    io::make_base_dirs();

    let key = "8cfb30b34977529853bbe46afdbbd5ae".to_string();
    let addr = String::from("127.0.0.1:8088");

    KEY.lock().unwrap().push(key);

    println!("listening on port {}",&addr);

    HttpServer::new(|| {
        App::new()
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
    })
    .bind(addr)?
    .run()

}

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

        match io::crypted::write(injson["file"].to_string(),combine.as_bytes().to_vec()) {
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
        let result: io::crypted::CRYPT;
        match io::crypted::read(file_name.to_string()) {
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
        match io::files::read(file_name.to_string()) {
            Ok(s) => {
                result = s;
            },
            Err(e) => {
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

        match io::files::write(injson["file"].to_string(),data.as_bytes().to_vec()) {
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

        let fetch = io::get_files(location.to_string());
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
