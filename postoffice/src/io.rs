use std::env;
use std::path::Path;
use std::fs::{create_dir_all,File,read_to_string,remove_file};
use std::io::Write;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[allow(dead_code)]
pub fn cwd() -> String {
    match env::current_dir() {
        Ok(path)=>{
            match path.to_str() {
                Some(str)=>{
                    return str.to_string();
                },
                None=>{
                    return String::new();
                }
            }
        },
        Err(_)=>{
            return String::new();
        }
    }
}

#[allow(dead_code)]
pub fn ensure_dir(path:&String) -> bool {
    match create_dir_all(path) {
        Ok(_)=>{
            return true;
        },
        Err(_)=>{
            return false;
        }
    }
}

#[allow(dead_code)]
pub fn check_path(path:&String) -> bool {
    Path::new(path).exists()
}

#[allow(dead_code)]
pub fn write(path:String,data:Vec<u8>) -> Result<(),String> {
    match File::create(&path) {
        Ok(mut file)=>{
            match file.write(&data) {
                Ok(_)=>{
                    return Ok(());
                },
                Err(e)=>{
                    let error = format!("failed-write_data-write_control=>{}",e);
                    return Err(error);
                }
            }
        },
        Err(_)=>{
            return Err("failed-open_file-write_control".to_string());
        }
    }
}

#[allow(dead_code)]
pub fn get_random_file_name() -> String {
    let connection_id: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .collect();
    return connection_id;
}

#[allow(dead_code)]
pub fn new_file(path:&String) -> bool {
    match File::create(path){
        Ok(_)=>{
            return true;
        },
        Err(_)=>{
            return false;
        }
    }
}

#[allow(dead_code)]
pub fn read(path:&String) -> Result<String,String> {
    match read_to_string(path){
        Ok(str)=>{
            return Ok(str);
        },
        Err(_)=>{
            return Err("failed to read the file as string".to_string());
        }
    }
}

pub fn delete_file(path:String) -> Result<(),String> {
    match remove_file(path) {
        Ok(_)=>{
            return Ok(());
        },
        Err(e)=>{
            let error = format!("failed-delete_file=>{}",e);
            return Err(error);
        }
    }
}
