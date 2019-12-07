#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use std::path::Path;
use std::env;

use std::io::Write;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
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
                web::resource("/write").route(web::post().to_async(write))
            )
            .service(
                web::resource("/read").route(web::post().to_async(read))
            )
    })
    .bind(addr)?
    .run()

}

fn write(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

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

        let current_dir_object = env::current_dir().unwrap();
        let current_dir = current_dir_object.to_str().unwrap();

        let base_dir_main = format!("{}\\files",current_dir);
        let base_dir_backup = format!("{}\\backup",current_dir);

        let create_main_dir = fs::create_dir_all(&base_dir_main);
        let create_backup_dir = fs::create_dir_all(&base_dir_backup);

        match create_main_dir {
            Ok(()) => {},
            Err(_e) => {
                return Ok(server::error("failed-create_main_dir".to_string()));
            }
        }

        match create_backup_dir {
            Ok(()) => {},
            Err(_e) => {
                return Ok(server::error("failed-create_backup_dir".to_string()));
            }
        }

        let location = format!("{}\\{}.fdbef",&base_dir_main,&injson["file"]);

        if Path::new(&location).exists() {
            let backup_location = format!("{}\\{}.fdbef",&base_dir_backup,&injson["file"]);
            let copy_backup_file = fs::copy(&location, &backup_location);
            match copy_backup_file {
                Ok(_r) => {},
                Err(_e) => {
                    return Ok(server::error("failed-copy_backup_file".to_string()));
                }
            }
        }

        let opener = &KEY.lock().unwrap();
        let key = &opener[0];

        let data_dump = &injson["data"].dump();
        let encrypted = crypt::encrypt(data_dump.to_string(),key.to_string());
        let combine = format!("{:?};{:?}",encrypted.nonce,encrypted.cipher);

        let f = File::create(&location);
        match f {
            Ok(mut r) => {
                match r.write(&combine.as_bytes()) {
                    Ok(_r) => {},
                    Err(_e) => {
                        return Ok(server::error("failed-write_file".to_string()));
                    }
                }
            },
            Err(_e) => {
                return Ok(server::error("failed-create_file".to_string()));
            }
        }

        let open_book = &mut BOOK.lock().unwrap();
        open_book.insert(injson["file"].to_string(),injson["data"].clone());

        return Ok(server::success());

    })

}

fn read(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

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
        let open_book = BOOK.lock().unwrap();
        if open_book.contains_key(file_name) {
            return Ok(server::success_with_data(open_book[file_name].clone()));
        }

        //read from file
        let current_dir_object = env::current_dir().unwrap();
        let current_dir = current_dir_object.to_str().unwrap();
        let base_dir_main = format!("{}\\files",current_dir);
        let location = format!("{}\\{}.fdbef",&base_dir_main,&injson["file"]);

        let f = File::open(&location);
        let mut buffer = Vec::new();
        match f {
            Ok(mut r) => {
                match r.read_to_end(&mut buffer) {
                    Ok(_r) => {},
                    Err(_e) => {
                        return Ok(server::error("failed-read_file-read_to_end".to_string()));
                    }
                }
            },
            Err(_e) => {
                return Ok(server::error("failed-read_file-open_file".to_string()));
            }
        }

        let as_string;
        match String::from_utf8(buffer) {
            Ok(v) => {
                as_string = v;
            },
            Err(_e) => {
                return Ok(server::error("failed-read_file-parse_data".to_string()));
            }
        }

        let mut collect = Vec::new();
        for hold in as_string.split(";") {
            collect.push(hold);
        }

        let nonce : Vec<u8> = collect[0]
        .trim_matches(|c| c == '[' || c == ']')
        .split(",")
        .map(|n| n.trim().parse().unwrap())
        .collect();

        let cipher : Vec<u8> = collect[1]
        .trim_matches(|c| c == '[' || c == ']')
        .split(",")
        .map(|n| n.trim().parse().unwrap())
        .collect();

        let opener = &KEY.lock().unwrap();
        let key = &opener[0];

        let decrypted = crypt::decrypt(cipher,key.to_string(),nonce);
        let result = json::parse(&decrypted);
        let injson: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };

        //println!("{:?}",injson);

        return Ok(server::success_with_data(injson));

    })

}
