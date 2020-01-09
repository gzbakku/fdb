use crate::common;
use crate::crypt;
use std::fs::File;
use std::io::Read;
use json::JsonValue;
use std::path::Path;
use std::env;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Instance {
    pub id:String,
    pub sig:String
}

impl Instance {
    pub fn new()->Instance{
        Instance{
            id:String::new(),
            sig:String::new()
        }
    }
    pub fn update(&mut self,i:&Instance){
        self.id = i.id.clone();
        self.sig = i.sig.clone();
    }
}

#[derive(Debug)]
pub struct Session {
    pub id:String,
    pub sig:String
}

impl Session {
    pub fn new()->Session{
        Session{
            id:String::new(),
            sig:String::new()
        }
    }
}

#[derive(Debug)]
pub struct Composer {
    pub id:String,
    pub sig:String,
    pub ip:String,
    pub port:String
}

#[derive(Debug)]
pub struct Node {
    pub id:String,
    pub sig:String,
    pub port:String
}

#[derive(Debug)]
pub struct Extracted {
    pub app_type:String,
    pub composer_ip:String,
    pub password:String,
    pub config:JsonValue,
    pub base_dir:String,
    pub instance:Instance,
    pub session:Session,
    pub composer:Composer,
    pub node:Node
}

impl Extracted {
    fn composer(&mut self) -> Composer {
        Composer {
            id:self.composer.id.clone(),
            ip:self.composer.ip.clone(),
            sig:self.composer.sig.clone(),
            port:self.composer.port.clone()
        }
    }
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
                                    match extract_from_config(&config) {
                                        Ok(r)=>{

                                            if check_path(&String::from(r["base_directory_location"].clone())) == false {
                                                return Err(common::error("does_not_exists-base_dir-definied_in_config"));
                                            }

                                            let mut node = Node {
                                                id:r["composer_id"].clone(),
                                                sig:r["composer_signature"].clone(),
                                                port:r["composer_port"].clone()
                                            };

                                            if r["type"] == "node" {
                                                match extract_from_node(&config) {
                                                    Ok(nr)=>{
                                                        node.id = nr["node_id"].clone();
                                                        node.port = nr["node_port"].clone();
                                                        node.sig = nr["node_signature"].clone()
                                                    },
                                                    Err(_)=>{
                                                        return Err(common::error("failed-extract_from_node"));
                                                    }
                                                }
                                            }

                                            let build = Extracted {
                                                app_type:r["type"].clone(),
                                                base_dir:r["base_directory_location"].clone(),
                                                password:secure,
                                                instance:Instance {
                                                    id:r["instance_id"].clone(),
                                                    sig:r["instance_signature"].clone()
                                                },
                                                session:Session {
                                                    id:common::uid(),
                                                    sig:common::uid()
                                                },
                                                composer:Composer {
                                                    ip:r["composer_ip"].clone(),
                                                    id:r["composer_id"].clone(),
                                                    sig:r["composer_signature"].clone(),
                                                    port:r["composer_port"].clone()
                                                },
                                                node:node,
                                                composer_ip:r["composer_ip"].clone(),
                                                config:config,
                                            };

                                            return Ok(build);

                                        },
                                        Err(_)=>{
                                            return Err(common::error("failed-extract_from_config"));
                                        }
                                    }
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

pub fn extract_from_config(config:&JsonValue) -> Result<HashMap<String,String>,String> {

    let extract_these = ["type","base_directory_location","instance_id","instance_signature","composer_id","composer_ip","composer_signature","composer_port"];

    extract_from_json(config,extract_these.to_vec())

}

fn extract_from_node(config:&JsonValue) -> Result<HashMap<String,String>,String> {

    let extract_these = ["node_id","node_port","node_signature"];

    extract_from_json(config,extract_these.to_vec())

}

fn extract_from_json(config:&JsonValue,extract_these:Vec<&str>) -> Result<HashMap<String,String>,String> {

    let mut collect = HashMap::new();

    for item in extract_these.iter() {
        let item_clone = item.clone();
        let item_as_string = String::from(item_clone);
        if item == &"composer_port" || item == &"node_port" {
            match config[item_clone].as_u16() {
                Some(r) => {
                    collect.insert(item_as_string,r.to_string());
                },
                None => {
                    return Err(common::error("failed-process_item-extract_from_config"));
                }
            }
        } else {
            match config[item_clone].as_str() {
                Some(r) => {
                    collect.insert(item_as_string,r.to_string());
                },
                None => {
                    return Err(common::error("failed-process_item-extract_from_config"));
                }
            }
        }
    }

    Ok(collect)

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
