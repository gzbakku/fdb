use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::path::Path;
use std::fs::{File,read_to_string};
use std::io::Write;
use json::{parse,JsonValue};

struct WRITER {
    active:i32,
    cuid:String,
    que:Vec<i32>
}

lazy_static! {
    static ref FILES:Mutex<HashMap<String,WRITER>> = Mutex::new(HashMap::new());
}

pub fn get_file_token(file_name:String) -> Result<(),String> {
    match FILES.lock() {
        Ok(map)=>{

        },
        Err(_)=>{
            return Err("failed-lock_file_writer_map".to_string());
        }
    }
}

pub fn ensure_file(path:&String) -> bool {
    Path::new(path).exists()
}

pub fn read_collection_control(path:String) -> Result<JsonValue,String> {
    match read_to_string(&path) {
        Ok(str)=>{
            match parse(&str) {
                Ok(obj)=>{
                    return Ok(obj);
                },
                Err(e)=>{
                    let error = format!("failed-parse_to_json=>{}",e);
                    return Err(error);
                }
            }
        },
        Err(e)=>{
            let error = format!("failed-open_file=>{}=>{}",path,e);
            return Err(error);
        }
    }
}

pub fn write(path:String,data:String) -> Result<(),String> {
    match File::create(&path) {
        Ok(mut file)=>{
            match file.write(data.as_bytes()) {
                Ok(_)=>{
                    return Ok(());
                },
                Err(e)=>{
                    let error = format!("failed-write_to_file=>{}",e);
                    return Err(error);
                }
            }
        },
        Err(e)=>{
            let error = format!("failed-open_file=>{}=>{}",path,e);
            return Err(error);
        }
    }
}

pub fn write_to_collection(path:String,data:String) -> Result<(),String> {

    Ok(())

}
