use std::fs::File;
use std::result;
use std::env::current_dir;
use std::io::{Write,Read};

#[derive(Debug)]
pub struct CRYPT {
    pub nonce:Vec<u8>,
    pub cipher:Vec<u8>
}

pub fn read(file_name:String) -> Result<CRYPT,String> {

    let current_dir_object = current_dir().unwrap();
    let current_dir = current_dir_object.to_str().unwrap();
    let location = format!("{}\\fdb\\vault\\{}.fdbv",current_dir,file_name);

    let f = File::open(&location);
    let mut buffer = Vec::new();
    match f {
        Ok(mut r) => {
            match r.read_to_end(&mut buffer) {
                Ok(_r) => {},
                Err(e) => {
                    println!("!!! failed-read_file-read-cypted-io error : {:?}",e);
                    return Err("!!! failed-read_file-read-cypted-io".to_string());
                }
            }
        },
        Err(e) => {
            println!("!!! failed-open_file-read-cypted-io error : {:?}",e);
            return Err("!!! failed-open_file-read-cypted-io".to_string());
        }
    }

    let as_string;
    match String::from_utf8(buffer) {
        Ok(v) => {
            as_string = v;
        },
        Err(e) => {
            println!("!!! failed-convert_array_to_string-read-cypted-io error : {:?}",e);
            return Err("!!! failed-convert_array_to_string-read-cypted-io".to_string());
        }
    }

    let mut collect = Vec::new();
    for hold in as_string.split(";") {
        collect.push(hold);
    }

    let nonce : Vec<u8> = collect[0]
    .trim_matches(|c| c == '[' || c == ']')
    .split(",")
    .map(|n| n.trim().parse().unwrap())
    .collect();

    let cipher : Vec<u8> = collect[1]
    .trim_matches(|c| c == '[' || c == ']')
    .split(",")
    .map(|n| n.trim().parse().unwrap())
    .collect();

    return Ok(CRYPT {
        nonce:nonce,
        cipher:cipher
    });

}

pub fn write(file_name:String,data:Vec<u8>) -> Result<(),String> {

    let current_dir_object = current_dir().unwrap();
    let current_dir = current_dir_object.to_str().unwrap();
    let location = format!("{}\\fdb\\vault\\{}.fdbv",current_dir,file_name);

    let f = File::create(&location);
    match f {
        Ok(mut r) => {
            match r.write(&data) {
                Ok(_r) => {
                    return Ok(());
                },
                Err(e) => {
                    println!("!!! failed-write_file-cypted-io error : {:?}",e);
                    return Err("!!! failed-write_file-cypted-io error".to_string());
                }
            }
        },
        Err(e) => {
            println!("!!! failed-create_file-cypted-io error : {:?}",e);
            return Err("!!! failed-create_file-cypted-io error".to_string());
        }
    }

}
