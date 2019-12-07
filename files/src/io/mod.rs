use std::result;
use std::fs::{ read_dir, create_dir_all, copy};
use std::env;

use json::JsonValue;

//use std::io;
use std::path::Path;

pub mod crypted;
pub mod list;
pub mod files;

pub fn backup(location:String,file_name:String,file_type:String) -> Result<(),String> {

    let current_dir_object = env::current_dir().unwrap();
    let current_dir = current_dir_object.to_str().unwrap();

    let file_location = format!("{}\\fdb\\{}\\{}.{}",current_dir,location,file_name,file_type);
    let backup_location = format!("{}\\fdb\\backup\\{}.{}",current_dir,file_name,file_type);

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

pub fn make_base_dirs() -> Result<(),String> {

    let current_dir_object = env::current_dir().unwrap();
    let current_dir = current_dir_object.to_str().unwrap();

    let base_dir_main = format!("{}\\fdb\\",current_dir);
    let base_dir_files = format!("{}\\fdb\\files\\",current_dir);
    let base_dir_list = format!("{}\\fdb\\list\\",current_dir);
    let base_dir_vault = format!("{}\\fdb\\vault\\",current_dir);
    let base_dir_backup = format!("{}\\fdb\\backup\\",current_dir);

    match create_dir_all(&base_dir_main) {
        Ok(_r) => {},
        Err(e) => {
            return Err(e.to_string());
        }
    }

    match create_dir_all(&base_dir_files) {
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

    match create_dir_all(&base_dir_vault) {
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

    return Ok(());

}

pub fn get_files(location:String) ->  Result<JsonValue,String> {

    let current_dir_object = env::current_dir().unwrap();
    let current_dir = current_dir_object.to_str().unwrap();
    let base_dir_main = format!("{}\\fdb\\{}\\",current_dir,location);

    if Path::new(&base_dir_main).exists() == false {
        return Err("directory_not_found-get_files-list-io".to_string());
    }

    let mut collect = json::JsonValue::new_array();

    for file in read_dir(base_dir_main).unwrap() {
        match file {
            Ok(r) => {
                collect.push(parse(r.path().to_str().unwrap().to_string()));
            },
            Err(_e) => {}
        }
    }

    return Ok(collect);

}

fn parse(location:String) -> String {
    let mut collect = Vec::new();
    for item in location.split("\\") {
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
