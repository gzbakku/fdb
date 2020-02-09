use easy_ssl::{builder,generate_as_files};
use crate::io::{extract_from_json,Node,ensure_dir};
use crate::common;
use json::{JsonValue};
use std::fs::File;

pub struct SSL{
    pub cert:String,
    pub key:String
}

pub fn check(config:&JsonValue,node:&Node,config_file_location:&String) -> Result<SSL,String> {

    match extract_ssl_info(config) {
        Ok(r)=>{
            return Ok(r);
        },
        Err(_)=>{
            common::log("generating new ssl certifcate and private key file");
            let mut clone = config.clone();
            match make_ssl_info(&mut clone,node,config_file_location) {
                Ok(r)=>{
                    return Ok(r);
                },
                Err(_)=>{
                    return Err(common::error("failed-make_ssl_info-check-ssl"));
                }
            }
        }
    }

}

fn make_ssl_info(config:&mut JsonValue,node:&Node,config_file_location:&String) -> Result<SSL,String> {

    match extract_from_json(&config,["base_directory_location"].to_vec()) {
        Ok(map)=>{

            let base_dir = map.get("base_directory_location").unwrap();
            let ssl_dir = format!("{}/ssl",base_dir);
            match ensure_dir(&ssl_dir) {
                Ok(_)=>{},
                Err(_)=>{
                    return Err(common::error("failed-ensure_ssl_dir-make_ssl_info"));
                }
            }

            let cert_path = format!("{}/cert.pem",ssl_dir);
            let key_path = format!("{}/key.pem",ssl_dir);
            let mut build = make_ssl_builder(cert_path.clone(),key_path.clone(),node);
            match generate_as_files(&mut build) {
                Ok(_)=>{
                    let paths = SSL {
                        cert:cert_path,
                        key:key_path
                    };
                    match update_config(config,&paths,config_file_location.to_string()) {
                        Ok(_)=>{
                            return Ok(paths);
                        },
                        Err(_)=>{
                            return Err(common::error("failed-update_config-make_ssl_info"));
                        }
                    }
                },
                Err(e)=>{
                    println!("error : {:?}",e);
                    return Err(common::error("failed-generate_as_files-make_ssl_info"));
                }
            }

        },
        Err(_)=>{
            return Err(common::error("failed-extract_from_json-make_ssl_info"));
        }
    }

}

fn update_config(config:&JsonValue,ssl:&SSL,config_file_location:String) -> Result<(),String> {

    let mut hold = config.clone();

    match hold.insert("ssl_certificate_file_path",ssl.cert.clone()) {
        Ok(_)=>{},
        Err(_)=>{
            return Err(common::error("failed-insert-ssl_certificate_file_path-injons"));
        }
    }

    match hold.insert("ssl_key_file_path",ssl.key.clone()) {
        Ok(_)=>{},
        Err(_)=>{
            return Err(common::error("failed-insert-ssl_key_file_path-injons"));
        }
    }

    match common::time::now() {
        Ok(t)=>{
            match hold.insert("ssl_certificate_timestamp",t) {
                Ok(_)=>{},
                Err(_)=>{
                    return Err(common::error("failed-insert-ssl_certificate_timestamp-injons"));
                }
            }
        },
        Err(_)=>{
            return Err(common::error("failed-fetch_time_now-update_config"));
        }
    }

    //println!("hold : {:?}",hold);

    match File::create(config_file_location) {
        Ok(mut f) => {
            match hold.write_pretty(&mut f, 2) {
                Ok(_) => {
                    common::log("config file updated");
                    return Ok(());
                },
                Err(_) => {
                    return Err(common::error("failed to update config file"));
                }
            }
        },
        Err(_) => {
            return Err(common::error("failed to open config file"));
        }
    }

}

fn make_ssl_builder(cert:String,key:String,node:&Node) -> builder::Builder {

    let mut build = builder::Builder::new();

    build.set_key_path(key);
    build.set_certificate_path(cert);
    build.set_key_size(2048);

    build.issuer.set_country("IN".to_string());
    build.issuer.set_state("UP".to_string());
    build.issuer.set_location("GZB".to_string());
    build.issuer.set_org("DAACHI".to_string());
    build.issuer.set_common_name("https://daachi.in".to_string());

    build.subject.set_country("IN".to_string());
    build.subject.set_state("UP".to_string());
    build.subject.set_location("GZB".to_string());
    build.subject.set_org("DAACHI".to_string());
    build.subject.set_common_name(node.ip.clone());

    return build;

}

fn extract_ssl_info(config:&JsonValue) -> Result<SSL,String> {

    let extract_these = ["ssl_certificate_file_path","ssl_key_file_path","ssl_certificate_timestamp"];

    match extract_from_json(config,extract_these.to_vec()) {
        Ok(map)=>{

            match common::time::now() {
                Ok(now_as_string)=>{

                    let now;
                    match now_as_string.parse::<i64>() {
                        Ok(r)=>{
                            now = r;
                        },
                        Err(_)=>{
                            return Err(common::error("failed-parse-current_timestamp-inconfig"));
                        }
                    }

                    let expire_as_string;
                    match map.get("ssl_certificate_timestamp") {
                        Some(r)=>{
                            expire_as_string = r;
                        },
                        None=>{
                            return Err(common::error("not_found-ssl_certificate_timestamp-inconfig"));
                        }
                    }

                    let expire;
                    match expire_as_string.to_string().parse::<i64>() {
                        Ok(r)=>{
                            expire = r;
                        },
                        Err(_)=>{
                            return Err(common::error("failed-parse-ssl_certificate_timestamp-inconfig"));
                        }
                    }

                    let day_365_in_unix_epoch:i64 = 360 * 24 * 60 * 60 * 1000;
                    let finish_line:i64 = expire + day_365_in_unix_epoch;

                    if finish_line <= now {
                        return Err(common::error("ssl-expired"));
                    }

                },
                Err(_)=>{
                    return Err(common::error("failed-fetch_current_time"));
                }
            }

            return Ok(SSL {
                cert:map.get("ssl_certificate_file_path").unwrap().to_string(),
                key:map.get("ssl_key_file_path").unwrap().to_string()
            });
        },
        Err(e)=>{
            return Err(common::error("failed-extract_ssl_info"));
        }
    }

}
