
use std::fs::File;
use std::io::{Write,Read};

use std::fs::{ read_dir, create_dir_all, copy};
use std::env;
use json::JsonValue;
use std::path::Path;
use crate::common;

pub mod crypted;
pub mod files;

#[allow(dead_code)]
pub fn backup(location:String,file_name:String,file_type:String) -> Result<(),String> {

    let current_dir_object = env::current_dir().unwrap();
    let current_dir = current_dir_object.to_str().unwrap();

    let file_location = format!("{}/fdb/{}/{}.{}",current_dir,location,file_name,file_type);
    let backup_location = format!("{}/fdb/backup/{}.{}",current_dir,file_name,file_type);

    match copy(&file_location, &backup_location) {
        Ok(_r) => {
            return Ok(());
        },
        Err(e) => {
            println!("!!! failed backup file");
            println!("!!! file_location : {:?}",file_location);
            println!("!!! backup_location : {:?}",backup_location);
            println!("!!! backup_file_Error : {:?}",e);
            return Err(e.to_string());
        }
    }

}

pub fn make_base_dirs(current_dir:&String) -> Result<(),String> {

    let base_dir_main = format!("{}/files/",current_dir);
    let base_dir_list = format!("{}/files/list/",current_dir);
    let base_dir_backup = format!("{}/files/backup/",current_dir);
    let base_dir_collections = format!("{}/files/collections/",current_dir);

    match create_dir_all(&base_dir_main) {
        Ok(_r) => {},
        Err(e) => {
            return Err(e.to_string());
        }
    }

    match create_dir_all(&base_dir_list) {
        Ok(_r) => {},
        Err(e) => {
            return Err(e.to_string());
        }
    }

    match create_dir_all(&base_dir_backup) {
        Ok(_r) => {},
        Err(e) => {
            return Err(e.to_string());
        }
    }

    match create_dir_all(&base_dir_collections) {
        Ok(_r) => {},
        Err(e) => {
            return Err(e.to_string());
        }
    }

    return Ok(());

}

pub fn check_path(p:&String) -> bool {
    if Path::new(p).exists() {
        return true;
    } else {
        return false;
    }
}

pub fn get_files(current_dir:String,location:String) ->  Result<JsonValue,String> {

    let base_dir_main = format!("{}/files/{}/",current_dir,location);

    if Path::new(&base_dir_main).exists() == false {
        return Err("directory_not_found-get_files-list-io".to_string());
    }

    let mut collect = json::JsonValue::new_array();

    for file in read_dir(base_dir_main).unwrap() {
        match file {
            Ok(r) => {

                match r.path().to_str() {
                    Some(e) => {
                        let parsed = parse(e.to_string());
                        match collect.push(parsed) {
                            Ok(_r) => {},
                            Err(_e) => {}
                        };
                    },
                    None => {}
                }

                //let parsed = parse(r.path().to_str().unwrap().to_string()
                //collect.push());
            },
            Err(_e) => {}
        }
    }

    return Ok(collect);

}

fn parse(location:String) -> String {
    let mut collect = Vec::new();
    for item in location.split("/") {
        collect.push(item.to_string());
    }
    let len = collect.len() - 1;
    let last = collect[len].to_string();
    let mut holder = Vec::new();
    for item in last.split(".") {
        holder.push(item.to_string());
    }
    return holder[0].to_string();
}

pub fn write(dir_path:String,file_path:String,data_as_json:JsonValue) -> Result<(),String> {

    let mut clone = data_as_json.clone();

    match create_dir_all(&dir_path) {
        Ok(_r) => {},
        Err(e) => {
            return Err(common::error_format(format!("failed-create_dir_all-files-io error : {:?}",e)));
        }
    }

    match File::create(&file_path) {
        Ok(mut r) => {
            match clone.write(&mut r) {
                Ok(_r) => {
                    return Ok(());
                },
                Err(e) => {
                    return Err(common::error_format(format!("failed-write_file-files-io error : {:?}",e)));
                }
            }
        },
        Err(e) => {
            return Err(common::error_format(format!("failed-create_file-files-io error : {:?}",e)));
        }
    }

}

pub fn read(file_path:String) -> Result<JsonValue,String> {

    let mut buffer = Vec::new();
    match File::open(&file_path) {
        Ok(mut r) => {
            match r.read_to_end(&mut buffer) {
                Ok(_r) => {},
                Err(e) => {
                    println!("!!! failed-read_file-read-files-io error : {:?} file_path : {:?}",e,file_path);
                    return Err("!!! failed-read_file-read-files-io".to_string());
                }
            }
        },
        Err(e) => {
            println!("!!! failed-open_file-read-files-io error : {:?} file_path : {:?}",e,file_path);
            return Err("!!! failed-open_file-read-files-io".to_string());
        }
    }

    match json::parse(std::str::from_utf8(&buffer).unwrap()) {
        Ok(data) => {
            return Ok(data);
        },
        Err(e) => {
            return Err(common::error("failed-parse_json"));
        },
    }

}
