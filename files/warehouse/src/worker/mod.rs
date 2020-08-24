use std::{thread,time};
use postoffice::collector;
use std::collections::HashMap;

pub mod disk;
pub mod collectify;
pub mod get;
mod actify;
pub mod list;

use collectify::FdbFile;

pub fn init(){
    thread::spawn(move || {
        loop{
            match process_collector(){
                Ok(name)=>{
                    // println!("process_collector success");
                    println!(">>> processing collection successfull : {}",name);
                },
                Err(e)=>{
                    if e == "no_collections_found"{
                        let sleep = time::Duration::from_millis(1000);
                        thread::sleep(sleep);
                        // println!("no_collections_found");
                    } else {
                        let sleep = time::Duration::from_millis(3000);
                        thread::sleep(sleep);
                        println!("process_collector failed : {:?}",e);
                    }
                }
            }
        }
    });
}

fn process_collector() -> Result<String,&'static str>{

    let mut collection:collector::collection::Collection;
    match collector::collection::get(false) {
        Ok(c)=>{
            collection = c;
        },
        Err(e)=>{
            if e == "no_collections_found"{
                return Err("no_collections_found");
            } else {
                println!("{:?}",e);
                return Err("failed-get_collection-process_collector-writer");
            }
        }
    }

    println!(">>> processing collection : {}",&collection.name);

    let files:HashMap<String,FdbFile>;
    match collectify::init(&mut collection){
        Ok(map)=>{
            files = map;
        },
        Err(e)=>{
            println!("{:?}",e);
            return Err("failed-parse_collection-process_collector-writer");
        }
    }

    if files.len() == 0{
        match collection.flush(){
            Ok(_)=>{
                return Ok(collection.name);
            },
            Err(_)=>{
                return Err("empty_collection-failed-flush_collection-process_collector-writer");
            }
        }
    }

    for val in files.values(){
        match process_file(val.clone()){
            Ok(_)=>{},
            Err(e)=>{
                println!("{:?}",e);
                return Err("failed-process_file-process_collector-writer");
            }
        }
    }

    match collection.flush(){
        Ok(_)=>{
            return Ok(collection.name);
        },
        Err(_)=>{
            return Err("failed-flush_collection-process_collector-writer");
        }
    }

}

fn process_file(file:FdbFile) -> Result<(),&'static str>{

    let handle: thread::JoinHandle<Result<(),&'static str>> = thread::spawn(move || {

        //get file lock
        let token:disk::Writer;
        match disk::get_token(&file.file_name, &file.file_type){
            Ok(t)=>{
                token = t;
            },
            Err(e)=>{
                println!("{:?}",e);
                return Err("failed-get_token-process_file-process_collector-writer");
            }
        }

        //parse file
        let mut map:disk::FdbMap;
        match disk::parse_map(&token){
            Ok(m)=>{
                map = m;
            },
            Err(e)=>{
                println!("{:?}",e);
                return Err("failed-parse_file-process_file-process_collector-writer");
            }
        }

        //update file
        let file_as_str:String;
        match actify::init(&file,&mut map){
            Ok(s)=>{
                file_as_str = s;
            },
            Err(e)=>{
                println!("{:?}",e);
                return Err("failed-actify_file-process_file-process_collector-writer");
            }
        }

        if file_as_str.len() == 0{
            match disk::delete_file_from_disk(&token){
                Ok(_)=>{
                    return Ok(());
                },
                Err(e)=>{
                    println!("{:?}",e);
                    return Err("failed-delete_file_from_disk-process_file-process_collector-writer");
                }
            }
        }

        //save file
        match disk::write_file(&token,file_as_str){
            Ok(_)=>{
                return Ok(());
            },
            Err(e)=>{
                println!("{:?}",e);
                return Err("failed-write_file-process_file-process_collector-writer");
            }
        }

    });

    match handle.join(){
        Ok(result)=>{
            match result{
                Ok(_)=>{
                    return Ok(());
                },
                Err(e)=>{
                    println!("{:?}",e);
                    return Err("failed-process_thread-handle_data");
                }
            }
        },
        Err(_)=>{
            return Err("failed-extract-handle_data");
        }
    }

}
