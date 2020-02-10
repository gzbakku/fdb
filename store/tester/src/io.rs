use std::fs::{File,read_to_string};
use std::env;
use std::io::Read;
use json::{JsonValue,parse};

pub fn cwd() -> String {
    match env::current_dir() {
        Ok(path)=>{
            match path.to_str() {
                Some(str)=>{
                    return str.to_string();
                },
                None=>{
                    return String::from("/");
                }
            }
        },
        Err(_)=>{
            return String::from("/");
        }
    }
}

pub fn read(path:String) -> Result<JsonValue,String> {

    match read_to_string(path) {
        Ok(json_string)=>{
            match parse(&json_string) {
                Ok(json)=>{
                    return Ok(json);
                },
                Err(_)=>{
                    return Err("failed to parse string to JsonValue for reading".to_string());
                }
            }
        },
        Err(e)=>{
            println!("base error : {:?}",e);
            return Err("failed to read the file as string".to_string());
        }
    }

}

pub fn write(path:String,data:JsonValue) -> Result<(),String> {

    match File::create(path) {
        Ok(mut file)=>{
            match data.write_pretty(&mut file,4) {
                Ok(_)=>{
                    return Ok(());
                },
                Err(_)=>{
                    return Err("failed to write data to file for writing".to_string());
                }
            }
        },
        Err(_)=>{
            return Err("failed to open file for writing".to_string());
        }
    }

}
