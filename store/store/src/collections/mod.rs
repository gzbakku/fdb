use std::thread;
use std::time::Duration;
use crate::{collector,validate};
use collector::collection::Collection;
use std::io::{BufReader,BufRead};
use std::fs::File;
use json::JsonValue;

pub fn process_collections(base_dir:&String) -> Result<(),String> {

    match collector::init(base_dir.to_string()) {
        Ok(_)=>{},
        Err(e)=>{
            let error = format!("failed-init_collector=>{}",e);
            return Err(error);
        }
    }

    loop_collections();

    return Ok(());

}

fn loop_collections(){

    thread::spawn(|| {
        loop {
            match collector::collection::get(false) {
                Ok(mut collection)=>{
                    match process_collection(&mut collection) {
                        Ok(_)=>{},
                        Err(e)=>{
                            println!("!!! failed-process_collection=>{}",e);
                            match collection.reset() {
                                Ok(_)=>{},
                                Err(e)=>{
                                    println!("!!! failed-reset_collection=>{}",e);
                                }
                            }
                            thread::sleep(Duration::from_millis(5000));
                        }
                    }
                },
                Err(e)=>{
                    if e != "no_collections_found" {
                        println!("!!! failed-reset_collection=>{:?}",e);
                    }
                    thread::sleep(Duration::from_millis(5000));
                }
            }
        }
    });

}

fn process_collection(collection:&mut Collection) -> Result<(),String> {

    println!("collection name : {:?}",collection.name);

    let reader:BufReader<File>;
    match collection.reader() {
        Ok(r)=>{
            reader = r;
        },
        Err(e)=>{
            let error = format!("!!! failed-get_collection_reader=>{}",e);
            return Err(error);
        }
    }

    let mut requests:Vec<JsonValue> = Vec::new();

    for liner in reader.lines() {
        match liner {
            Ok(line)=>{
                match validate::line(line) {
                    Ok(object)=>{
                        requests.push(object);
                    },
                    Err(e)=>{
                        let error = format!("failed-validate_line=>{}",e);
                        return Err(error);
                    }
                }
            },
            Err(e)=>{
                let error = format!("failed-extract_line=>{}",e);
                return Err(error);
            }
        }
    }

    println!("requests : {:?}",requests.len());

    return Err("no error".to_string());

    match collection.flush() {
        Ok(_)=>{},
        Err(e)=>{
            let error = format!("!!! failed-reset_collection=>{}",e);
            return Err(error);
        }
    }

    return Ok(());

}
