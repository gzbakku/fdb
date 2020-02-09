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
use json::{JsonValue,object};

mod resp;
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
pub struct Node_Template {
    pub id:String,
    pub sig:String,
    pub port:String
}

impl Node_Template {
    fn copy(&self) -> Node_Template {
        Node_Template {
            id:self.id.clone(),
            sig:self.sig.clone(),
            port:self.port.clone()
        }
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct Composer_Template {
    pub id:String,
    pub sig:String,
    pub ip:String,
    pub port:String
}

impl Composer_Template {
    fn copy(&self) -> Composer_Template {
        Composer_Template {
            id:self.id.clone(),
            sig:self.sig.clone(),
            ip:self.ip.clone(),
            port:self.port.clone()
        }
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
struct Session_Template {
    id:String,
    sig:String
}

impl Session_Template {
    fn copy(&self) -> Session_Template {
        Session_Template {
            id:self.id.clone(),
            sig:self.sig.clone()
        }
    }
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
    static ref NODE: Mutex<Node_Template> =  Mutex::new(Node_Template {
        id:String::new(),
        sig:String::new(),
        port:String::new()
    });
    static ref COMPOSER: Mutex<Composer_Template> =  Mutex::new(Composer_Template {
        id:String::new(),
        sig:String::new(),
        ip:String::new(),
        port:String::new()
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
                                 clap::Arg::with_name("actor_id")
                                  .help("actor id")
                                  .long("actor_id")
                                  .value_name("actor_id")
                                  .required(true)
                              )
                              .arg(
                                  clap::Arg::with_name("actor_signature")
                                   .help("actor secure signature")
                                   .long("actor_signature")
                                   .value_name("actor_signature")
                                   .required(true)
                               )
                               .arg(
                                   clap::Arg::with_name("node_port")
                                    .help("node_port id")
                                    .long("node_port")
                                    .value_name("node_port")
                                    .required(true)
                                )
                               .arg(
                                   clap::Arg::with_name("node_id")
                                    .help("node id")
                                    .long("node_id")
                                    .value_name("node_id")
                                    .required(true)
                                )
                                .arg(
                                    clap::Arg::with_name("node_signature")
                                     .help("node secure signature")
                                     .long("node_signature")
                                     .value_name("node_signature")
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
                                    clap::Arg::with_name("composer_ip")
                                     .help("composer ip adress and port")
                                     .long("composer_ip")
                                     .value_name("composer_ip")
                                     .required(true)
                                 )
                                 .arg(
                                     clap::Arg::with_name("composer_id")
                                      .help("composer_id")
                                      .long("composer_id")
                                      .value_name("composer_id")
                                      .required(true)
                                  )
                                  .arg(
                                      clap::Arg::with_name("composer_port")
                                       .help("composer_port")
                                       .long("composer_port")
                                       .value_name("composer_port")
                                       .required(true)
                                   )
                                  .arg(
                                      clap::Arg::with_name("composer_signature")
                                       .help("composer_signature")
                                       .long("composer_signature")
                                       .value_name("composer_signature")
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
                                   .arg(
                                       clap::Arg::with_name("cert_path")
                                        .help("x509 certificate file path")
                                        .long("cert_path")
                                        .value_name("cert_path")
                                        .required(true)
                                    )
                                    .arg(
                                        clap::Arg::with_name("key_path")
                                         .help("x509 private_key file path")
                                         .long("key_path")
                                         .value_name("key_path")
                                         .required(true)
                                     )
                          .get_matches();

        //*************************************************************
        //actor

        if matches.is_present("actor_id") {
            match matches.value_of("actor_id") {
                Some(id) => {
                    ACTOR.lock().unwrap().id = id.to_string();
                },
                None => {
                    common::error("not_found-actor_id");
                    return;
                }
            }
        }

        if matches.is_present("actor_signature") {
            match matches.value_of("actor_signature") {
                Some(signature) => {
                    ACTOR.lock().unwrap().sig = signature.to_string();
                },
                None => {
                    common::error("not_found-actor_signature");
                    return;
                }
            }
        }

        //*************************************************************
        //composer

        if matches.is_present("composer_id") {
            match matches.value_of("composer_id") {
                Some(id) => {
                    COMPOSER.lock().unwrap().id = id.to_string();
                },
                None => {
                    common::error("not_found-composer_id");
                    return;
                }
            }
        }

        if matches.is_present("composer_ip") {
            match matches.value_of("composer_ip") {
                Some(ip) => {
                    COMPOSER.lock().unwrap().ip = ip.to_string();
                },
                None => {
                    common::error("not_found-composer_ip");
                    return;
                }
            }
        }

        if matches.is_present("composer_port") {
            match matches.value_of("composer_port") {
                Some(port) => {
                    COMPOSER.lock().unwrap().port = port.to_string();
                },
                None => {
                    common::error("not_found-composer_port");
                    return;
                }
            }
        }

        if matches.is_present("composer_signature") {
            match matches.value_of("composer_signature") {
                Some(signature) => {
                    COMPOSER.lock().unwrap().sig = signature.to_string();
                },
                None => {
                    common::error("not_found-composer_signature");
                    return;
                }
            }
        }

        //*************************************************************
        //node

        if matches.is_present("node_id") {
            match matches.value_of("node_id") {
                Some(id) => {
                    NODE.lock().unwrap().id = id.to_string();
                },
                None => {
                    common::error("not_found-node_id");
                    return;
                }
            }
        }

        if matches.is_present("node_port") {
            match matches.value_of("node_port") {
                Some(port) => {
                    NODE.lock().unwrap().port = port.to_string();
                },
                None => {
                    common::error("not_found-node_port");
                    return;
                }
            }
        }

        if matches.is_present("node_signature") {
            match matches.value_of("node_signature") {
                Some(signature) => {
                    NODE.lock().unwrap().sig = signature.to_string();
                },
                None => {
                    common::error("not_found-node_signature");
                    return;
                }
            }
        }

        //*************************************************************
        //session

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

        let session_sig_string = format!("{}{}",ACTOR.lock().unwrap().sig,SESSION.lock().unwrap().sig);
        let session_hash = common::hash(session_sig_string);

        BASE_KEY.lock().unwrap().push(session_hash.to_string());

        //------------------------------------
        //ssl certs

        let mut cert_path = String::new();
        if matches.is_present("cert_path") {
            match matches.value_of("cert_path") {
                Some(key) => {
                    let key_as_string = String::from(key);
                    if io::check_path(&key_as_string) == false {
                        common::error("cert_path not found");
                        return;
                    } else {
                        cert_path = key_as_string;
                    }
                },
                None => {
                    common::error("not_found-cert_path");
                    return;
                }
            }
        }

        let mut key_path = String::new();
        if matches.is_present("key_path") {
            match matches.value_of("key_path") {
                Some(key) => {
                    let key_as_string = String::from(key);
                    if io::check_path(&key_as_string) == false {
                        common::error("key_path not found");
                        return;
                    } else {
                        key_path = key_as_string;
                    }
                },
                None => {
                    common::error("not_found-key_path");
                    return;
                }
            }
        }

        //------------------------------------
        //extract port

        if matches.is_present("port") {
            match matches.value_of("port") {
                Some(port) => {
                    match server(port.to_string(),cert_path,key_path) {
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
    let cert_path = String::from("D://workstation/expo/rust/fdb/cert/keys/cert.pem");
    let key_path = String::from("D://workstation/expo/rust/fdb/cert/keys/key.pem");
    DIR.lock().unwrap().push(dir);

    let get_dir = DIR.lock().unwrap()[0].to_string();

    match io::make_base_dirs(&get_dir) {
        Ok(_r) => {},
        Err(_e) => {
            panic!("failed-make_base_dirs-initiate-files-fdb");
        }
    }

    match server("8088".to_string(),cert_path,key_path) {
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
        let headers = req.headers();
        let path = req.path();
        let path_as_string = String::from(path);
        let base_key = BASE_KEY.lock().unwrap()[0].to_string();
        let node_as_mutex = NODE.lock().unwrap();
        let node_as_template = node_as_mutex.copy();
        let composer_as_mutex = COMPOSER.lock().unwrap();
        let composer_as_template = composer_as_mutex.copy();

        let peer = connection_info.remote();

        let mut access_granted = true;

        let mut peer_as_string:String = String::new();
        match auth::extract_peer(peer) {
            Ok(r)=>{
                peer_as_string = r.to_string();
            },
            Err(_)=>{
                common::error("failed-extract-peer-authenticate-files");
                access_granted = false;
            }
        }

        if access_granted {
            match auth::check(headers,base_key,node_as_template,peer_as_string,path_as_string,composer_as_template) {
                Ok(_)=>{},
                Err(_) => {
                    access_granted = false;
                }
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

fn server(port:String,cert_path:String,key_path:String) -> std::io::Result<()> {

    let addr = String::from(format!("127.0.0.1:{}",port));
    println!("@@@ listening on port {}",&addr);

    //common::success();

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
            return Ok(resp::error("not_found-file_name".to_string()));
        }
        if &injson.has_key("data") == &false {
            return Ok(resp::error("not_found-file_data".to_string()));
        }
        if &injson["data"].is_object() == &false {
            return Ok(resp::error("invalid-file_data".to_string()));
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
                return Ok(resp::error(e.to_string()));
            }
        }

        let open_book = &mut BOOK.lock().unwrap();
        open_book.insert(injson["file"].to_string(),injson["data"].clone());

        return Ok(resp::success());

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
            return Ok(resp::error("invalid_request-read_file".to_string()));
        }
        if &injson.has_key("file") == &false {
            return Ok(resp::error("not_found-read_file-file_name".to_string()));
        }

        //check book
        let file_name = &injson["file"].to_string();
        let open_book = &mut BOOK.lock().unwrap();
        if open_book.contains_key(file_name) {
            return Ok(resp::success_with_data(open_book[file_name].clone()));
        }

        //read from file
        let get_dir = DIR.lock().unwrap()[0].to_string();
        let result: io::crypted::CRYPT;
        match io::crypted::read(get_dir,file_name.to_string()) {
            Ok(r) => {
                result = r;
            },
            Err(e) => {
                return Ok(resp::error(e.to_string()));
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
            return Ok(resp::error("failed-parse_decrypted_into_object".to_string()));
        }

        open_book.insert(file_name.to_string(),data.clone());

        return Ok(resp::success_with_data(data));

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
            return Ok(resp::error("invalid_request-read_file".to_string()));
        }
        if &injson.has_key("file") == &false {
            return Ok(resp::error("not_found-read_file-file_name".to_string()));
        }

        //check book
        let file_name = &injson["file"].to_string();
        let open_book = &mut BOOK.lock().unwrap();
        if open_book.contains_key(file_name) {
            return Ok(resp::success_with_data(open_book[file_name].clone()));
        }

        //read from file
        let result: String;
        let get_dir = DIR.lock().unwrap()[0].to_string();
        match io::files::read(get_dir,file_name.to_string()) {
            Ok(s) => {
                result = s;
            },
            Err(_e) => {
                return Ok(resp::error("failed-read_file-read_json".to_string()));
            }
        }

        let result = json::parse(&result);
        let data: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };

        if &data.has_key("err") == &true {
            return Ok(resp::error("failed-parse_decrypted_into_object".to_string()));
        }

        open_book.insert(file_name.to_string(),data.clone());

        return Ok(resp::success_with_data(data));

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
            return Ok(resp::error("not_found-file_name".to_string()));
        }
        if &injson.has_key("data") == &false {
            return Ok(resp::error("not_found-file_data".to_string()));
        }
        if &injson["data"].is_object() == &false {
            return Ok(resp::error("invalid-file_data".to_string()));
        }

        let data = injson["data"].dump();
        let get_dir = DIR.lock().unwrap()[0].to_string();

        match io::files::write(get_dir,injson["file"].to_string(),data.as_bytes().to_vec()) {
            Ok(_r) => {},
            Err(e) => {
                return Ok(resp::error(e.to_string()));
            }
        }

        let open_book = &mut BOOK.lock().unwrap();
        open_book.insert(injson["file"].to_string(),injson["data"].clone());

        return Ok(resp::success());

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
            return Ok(resp::error("not_found-file_name-list".to_string()));
        }
        if &injson["location"].is_string() == &false {
            return Ok(resp::error("invalid-file_data-list".to_string()));
        }

        let location = &injson["location"].to_string();

        if
            location != &"files".to_string() &&
            location != &"vault".to_string() &&
            location != &"list".to_string()
        {
            return Ok(resp::error("invalid-location-list".to_string()));
        }

        let get_dir = DIR.lock().unwrap()[0].to_string();

        let fetch = io::get_files(get_dir,location.to_string());
        match fetch {
            Ok(r) => {
                return Ok(resp::success_with_data(r));
            },
            Err(e) => {
                return Ok(resp::error(e.to_string()));
            }
        }

    })

}

fn check(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|_body| {
        let session_as_mutex = SESSION.lock().unwrap();
        let session_as_template = session_as_mutex.copy();
        let session_id = session_as_template.id;
        let session_as_json = object!{
            "session_id" => session_id
        };
        return Ok(resp::success_with_data(session_as_json));
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
        return Ok(resp::success());
    })

}
