use crate::common;
use crate::crypt;
use std::fs::File;
use std::io::Read;
use json::JsonValue;
use std::path::Path;
use std::env;

#[derive(Debug)]
pub struct Extracted {
    pub password:String,
    pub config:JsonValue,
    pub base_dir:String
}

pub fn read_config(config_location:String,password:String) -> Result<Extracted,String> {

    let cipher:Vec<u8>;
    let nonce:Vec<u8>;
    let config:JsonValue;

    match File::open(config_location) {
        Ok(mut r) => {
            let mut object_as_string = String::new();
            match r.read_to_string(&mut object_as_string) {
                Ok(_) => {
                    match json::parse(&object_as_string){
                        Ok(object) => {
                            config = object.clone();
                            cipher = convert_json_string_to_vec(object["cipher"].to_string());
                            nonce = convert_json_string_to_vec(object["nonce"].to_string());
                            match crypt::extract_password(password,nonce,cipher) {
                                Ok(secure) => {
                                    //check if base directory defined in config exists
                                    let get_base_dir_path;
                                    match config["base_directory_location"].as_str() {
                                        Some(r) => {
                                            get_base_dir_path = r;
                                        },
                                        None => {
                                            return Err(common::error("failed-parse-base_dir-definied_in_config"));
                                        }
                                    }
                                    if check_path(&String::from(get_base_dir_path)) == false {
                                        return Err(common::error("does_not_exists-base_dir-definied_in_config"));
                                    }
                                    Ok(Extracted {
                                        base_dir:get_base_dir_path.to_string(),
                                        password:secure,
                                        config:config
                                    })
                                },
                                Err(_) => {
                                    Err(common::error("failed to parse config file into a json object"))
                                }
                            }
                        }
                        Err(_) => {
                            Err(common::error("failed to parse config file into a json object"))
                        }
                    }
                },
                Err(_) => {
                    Err(common::error("failed to read config file as a string"))
                }
            }
        },
        Err(e) => {
            println!("{:?}",e);
            Err(common::error("failed to read config file"))
        }
    }

}

pub fn check_path(path:&String) -> bool {
    if Path::new(&path).exists() {
        true
    } else {
        false
    }
}

pub fn current_dir() -> String {
    let current_dir_object = env::current_dir().unwrap();
    // let unparsed_current_dir = current_dir_object.to_str().unwrap();
    // let current_dir = unparsed_current_dir.replace("\\","/");
    let as_string = current_dir_object.to_str().unwrap();
    return as_string.to_string();
}

pub fn app_dir() -> String {
    let as_object = std::env::current_exe().unwrap();
    let as_string = as_object.to_str().unwrap();
    let collected = &as_string.split("composer").collect::<Vec<&str>>();
    let file_ext = collected[collected.len() - 1];
    let replace_this = format!("composer{}",file_ext);
    let final_dir = as_string.replace(&replace_this,"");
    return final_dir;
}

fn convert_json_string_to_vec(vector_as_string:String) -> Vec<u8> {
    let mut vec_str = vector_as_string.clone();
    vec_str = vec_str.replace("[","");
    vec_str = vec_str.replace("]","");
    let mut vec = Vec::new();
    for num in vec_str.split(",") {
        vec.push(num.parse::<u8>().unwrap());
    }
    return vec;
}
