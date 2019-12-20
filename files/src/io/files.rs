use std::fs::File;
use std::io::{Write,Read};

pub fn read(current_dir:String,file_name:String) -> Result<String,String> {

    let location = format!("{}/files/files/{}.fdbf",current_dir,file_name);

    let f = File::open(&location);
    let mut buffer = Vec::new();
    match f {
        Ok(mut r) => {
            match r.read_to_end(&mut buffer) {
                Ok(_r) => {},
                Err(e) => {
                    println!("!!! failed-read_file-read-files-io error : {:?}",e);
                    return Err("!!! failed-read_file-read-files-io".to_string());
                }
            }
        },
        Err(e) => {
            println!("!!! failed-open_file-read-files-io error : {:?}",e);
            return Err("!!! failed-open_file-read-files-io".to_string());
        }
    }

    let as_string;
    match String::from_utf8(buffer){
        Ok(v) => {
            as_string = v;
        },
        Err(e) => {
            println!("!!! failed-convert_array_to_string-read-files-io error : {:?}",e);
            return Err("!!! failed-convert_array_to_string-read-files-io".to_string());
        }
    }

    return Ok(as_string);

}

pub fn write(current_dir:String,file_name:String,data:Vec<u8>) -> Result<(),String> {

    let location = format!("{}/files/files/{}.fdbf",current_dir,file_name);

    let f = File::create(&location);
    match f {
        Ok(mut r) => {
            match r.write(&data) {
                Ok(_r) => {
                    return Ok(());
                },
                Err(e) => {
                    println!("!!! failed-write_file-files-io error : {:?}",e);
                    return Err("!!! failed-write_file-files-io".to_string());
                }
            }
        },
        Err(e) => {
            println!("!!! failed-create_file-files-io error : {:?}",e);
            return Err("!!! failed-create_file-files-io".to_string());
        }
    }

}
