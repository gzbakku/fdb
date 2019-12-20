extern crate rand;
extern crate reqwest;

use std::net::TcpListener;
use serde_json::json;
use serde_json::Value;
use serde::Deserialize;
use std::fs::{File,create_dir_all,remove_file,remove_dir_all};
use std::io;
use std::io::Read;
use crate::common;
use crate::crypt;

#[derive(Deserialize, Debug)]
struct IP {
    ip:String
}

pub fn init(base_ip:&String,base_password:&String,base_dir_location:&String,config_location:&String){

    common::space();
    common::log("initiating new fdb instance");

    //check for config file
    //process the password
    //get ip address
    //make the composing actors
    //create the config file
    //create the base directory

    //------------------------------------------------
    //make base object

    let mut config = json!({});

    //------------------------------------------------
    //process password here

    let digest = common::hash(base_password.to_string());
    let secure_password: String = common::uid();

    let copy_secure_password = secure_password.clone();

    let password = crypt::encrypt(secure_password,digest.to_string());

    config["nonce"] = serde_json::to_value(&password.nonce).unwrap();
    config["cipher"] = serde_json::to_value(&password.cipher).unwrap();

    common::log("prima-hashed key generated for encryption");

    if true {

        common::question("would you like to get the secure password generated for backup purposes if you do type ===> 'yes'");

        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm).expect("Failed to read from stdin");

        if confirm == "yes\r\n".to_string() {
            common::line();
            common::log_string(format!("Secure Randomly Generated Password : {:?}",copy_secure_password));
            common::answer();
        }

    }

    //------------------------------------------------
    //get ip address

    let ip:String;

    if base_ip == &"".to_string() {

        common::question("you have not provided any ip address would you like us to fetch one from an external api via https://api6.ipify.org?format=json if you do then type ===> 'yes'");

        let mut confirm_fetch_ip = String::new();
        io::stdin().read_line(&mut confirm_fetch_ip).expect("Failed to read from stdin");

        if confirm_fetch_ip == "yes\r\n".to_string() {

            let client = reqwest::Client::new();
            let mut fetch_ip_failed = true;

            match client.get("https://api6.ipify.org?format=json").send() {
                Ok(mut res) => {
                    match res.text() {
                        Ok(r) => {
                            match json::parse(&r) {
                                Ok(object) => {
                                    match object["ip"].as_str() {
                                        Some(origin) => {
                                            ip = origin.to_string();
                                            fetch_ip_failed = false;
                                        },
                                        None => {
                                            common::error("failed extract origin object");
                                            return;
                                        }
                                    }
                                },
                                Err(e) => {
                                    println!("{:#?}",e);
                                    common::error("failed to parse fetch ip result into json object");
                                    return;
                                }
                            }
                        },
                        Err(e) => {
                            println!("{:#?}",e);
                            common::error("failed to get current ip address");
                            return;
                        }
                    }
                },
                Err(e) => {
                    println!("{:#?}",e);
                    common::error("failed to fetch ip address via given api.");
                    common::error("check your internet connection.");
                    return;
                }
            }

        } else {
            common::error("please provide a valid ip adress or permission to fetch you ip from the given api serve to continue.");
            return;
        }

    } else {
        ip = base_ip.to_string();
    }

    common::line();
    common::log_string(format!("your ip : {:?}",ip));
    common::answer();

    let device_id: String = common::uid();
    let device_signature: String = common::uid();
    let instance_id: String = common::uid();
    let instance_signature: String = common::uid();

    config["app"] = serde_json::to_value("fdb".to_string()).unwrap();
    config["type"] = serde_json::to_value("composer".to_string()).unwrap();
    config["composer_ip"] = serde_json::to_value(ip).unwrap();
    config["device_id"] = serde_json::to_value(device_id).unwrap();
    config["device_signature"] = serde_json::to_value(device_signature).unwrap();
    config["instance_id"] = serde_json::to_value(instance_id).unwrap();
    config["instance_signature"] = serde_json::to_value(instance_signature).unwrap();
    config["config_file_location"] = serde_json::to_value(config_location).unwrap();
    config["base_directory_location"] = serde_json::to_value(base_dir_location).unwrap();

     //------------------------------------------------
     //make composing actors here

     let actors = [
        "composer",
        "files",
        "list",
        "dictionary",
        "users"
     ];

     let available_ports = get_ports(actors.len());
     let mut assigned_actors = Vec::new();
     let mut port_index:usize = 0;
     for a in &actors {
         let mut port_base = json!({});
         port_base["type"] = serde_json::to_value(a).unwrap();
         port_base["port"] = serde_json::to_value(available_ports[port_index]).unwrap();
         let actor_id: String = common::uid();
         port_base["id"] = serde_json::to_value(actor_id).unwrap();
         let actor_signature: String = common::uid();
         port_base["signature"] = serde_json::to_value(actor_signature).unwrap();
         port_index += 1;
         assigned_actors.push(port_base);
     }

     config["actors"] = serde_json::to_value(assigned_actors).unwrap();

     common::log("actors configured");

     //------------------------------------------------
     //make base directory

     match create_dir_all(&base_dir_location) {
         Ok(_) => {
             common::log("base directory generated");
         },
         Err(e) => {
             common::error("failed create fdb base directory");
             panic!("failed create fdb base directory");
         }
     }

     //------------------------------------------------
     //write json file

     match File::create(config_location) {
         Ok(f) => {
             match serde_json::to_writer_pretty(&f, &config) {
                 Ok(_) => {
                     common::log("config file generated");
                 },
                 Err(_) => {
                     common::error("failed to write data to config file");
                     panic!("failed to write data to config file");
                 }
             }
         },
         Err(_) => {
             common::error("failed create fdb config file");
             panic!("failed create fdb config file");
         }
     }

     common::space();

}

pub fn reset(base_ip:&String,base_password:&String,base_dir_location:&String,config_location:&String){

    //del the old file
    //del the base dir
    //initiate new instance

    common::space();
    common::log("resetting fdb instance");
    common::question("warning : this commond will delete all data from the old fdb instance and you will never be able to access it ever again would you like to continue if you do then type ===> 'yes' , process with caution");

    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).expect("Failed to read from stdin");

    //common::line();
    common::answer();

    if confirm != "yes\r\n".to_string() {
        common::error("user denied the confirmation");
        return;
    }

    common::log("removing base directory");

    match remove_dir_all(base_dir_location) {
        Ok(_) => {
            init(base_ip,base_password,base_dir_location,config_location);
        },
        Err(e) => {
            println!("{:?}",e);
            common::error("failed delete base directory");
            panic!("failed delete base directory");
        }
    }

    common::space();

}

pub fn node(base_ip:&String,base_password:&String,base_dir_location:&String,config_location:&String){

    //read config file
    //decrypt secure password
    //make base ids
    //sing the node config file

    common::space();
    common::log("creating new node");

    //*********************************
    //read config file

    let cipher:Vec<u8>;
    let nonce:Vec<u8>;
    let config;

    match File::open(config_location) {
        Ok(mut r) => {
            let mut object_as_string = String::new();
            match r.read_to_string(&mut object_as_string) {
                Ok(r) => {
                    match json::parse(&object_as_string){
                        Ok(object) => {
                            config = object.clone();
                            cipher = convert_json_string_to_vec(object["cipher"].to_string());
                            nonce = convert_json_string_to_vec(object["nonce"].to_string());
                        },
                        Err(e) => {
                            println!("{:?}",e);
                            common::error("failed to parse config file into a json object");
                            return;
                        }
                    }
                },
                Err(_) => {
                    common::error("failed to read config file as a string");
                    return;
                }
            }
        },
        Err(e) => {
            println!("{:?}",e);
            common::error("failed to read config file");
            return;
        }
    }

    let hash = common::hash(base_password.to_string());
    let password = crypt::decrypt(cipher,hash.to_string(),nonce);

    //*********************************
    //make node config here

    let mut node = json!({});

    node["app"] = serde_json::to_value("fdb".to_string()).unwrap();
    node["type"] = serde_json::to_value("node".to_string()).unwrap();
    node["node_ip"] = serde_json::to_value(base_ip.to_string()).unwrap();
    node["node_id"] = serde_json::to_value(common::uid()).unwrap();

    node["node_signature"] = serde_json::to_value(common::uid()).unwrap();

    node["base_directory_location"] = serde_json::to_value(base_dir_location.to_string()).unwrap();

    node["instance_signature"] = serde_json::to_value(config["instance_signature"].to_string()).unwrap();

    node["instance_id"] = serde_json::to_value(config["instance_id"].to_string()).unwrap();

    match common::time::now() {
        Ok(timestamp)=>{
            node["timestamp"] = serde_json::to_value(timestamp).unwrap();
        },
        Err(e)=>{
            println!("{:?}",e);
            common::error("failed to fetch time since unix epoch as mili seconds");
            return;
        }
    }

    let signature = format!(
        "{}-{}-{}-{}",
        node["node_id"],
        node["node_ip"],
        config["instance_id"].to_string(),
        password
    );

    node["auth_signature"] = serde_json::to_value(common::hash(signature)).unwrap();

    println!("{:#?}",node);

}

fn get_ports(how_many:usize) -> Vec<u16> {
    let mut available : Vec<u16> = Vec::new();
    let mut last : u16 = 5200;
    while available.len() <= how_many {
        match TcpListener::bind(("127.0.0.1", last)) {
            Ok(_) => {
                available.push(last);
                last += 1;
            },
            Err(_) => {
                last += 1;
            }
        }
    }
    available
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
