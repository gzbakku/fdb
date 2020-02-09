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
mod server;

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

pub fn init(){

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
                    match resp(port.to_string(),cert_path,key_path) {
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



//*******************************************************
//server functions

//*******************************************************
//server routes
