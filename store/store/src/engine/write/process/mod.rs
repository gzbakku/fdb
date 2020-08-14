use crate::engine::write::parse::Collection;
use std::collections::HashMap;
use std::thread;
use thread::JoinHandle;
use crate::collections;

mod define;
use define::{WriteResult,ERROR,SUCCESS};
use WriteResult::{Error,Success};

pub fn init(map:HashMap<String,Collection>) -> Result<(),String> {

    let mut collect:Vec<JoinHandle<WriteResult>> = Vec::new();

    for (_,collection) in map.iter() {
        if true {   //process only one collection
            if collect.len() == 0 {
                collect.push(write_collection(&collection));
            }
        } else {
            collect.push(write_collection(&collection));
        }
    }

    let mut results:Vec<WriteResult> = Vec::new();
    for handle in collect {
        match handle.join() {
            Ok(r)=>{
                results.push(r);
            },
            Err(_)=>{}
        }
    }

    let mut failed:Vec<String> = Vec::new();
    let mut success:Vec<String> = Vec::new();

    for result in results {
        match result {
            Success(s)=>{
                success.push(s.collection);
            },
            Error(e)=>{
                failed.push(e.collection);
            }
        }
    }

    println!("success : {:?}",success.len());
    println!("failed : {:?}",failed.len());

    return Err("no_error".to_string());

}

fn write_collection(collection_ref:&Collection) -> JoinHandle<WriteResult> {

    let collection = collection_ref.clone();

    thread::spawn(move || {

        let controller:collections::Collection;
        match collections::read_controller(&collection.cuid) {
            Ok(cont)=>{
                controller = cont;
            },
            Err(_)=>{
                return WriteResult::Error(ERROR::new_str(collection,"failed-fetch-collection_controller"));
            }
        }

        //println!("controller : {:?}",controller);
        println!("collection : {:?}",collection);


        return WriteResult::Error(ERROR::new_str(collection,"no_error"));

    })

}
