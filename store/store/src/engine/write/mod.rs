use json::JsonValue;
use std::collections::HashMap;
use md5;
pub mod parse;
mod process;

pub fn init(pool:Vec<JsonValue>) -> Result<(),String> {

    let mut collections:HashMap<String,parse::Collection> = HashMap::new();

    for request in pool.iter() {
        if true {
            for collection in request["data"].entries() {
                match parse::process_collection(collection.1) {
                    Ok(coll)=>{
                        if collections.contains_key(&coll.cuid) {
                            for file in &coll.files {
                                match collections.get_mut(&coll.cuid) {
                                    Some(pooler)=>{
                                        pooler.files.push(file.to_string());
                                    },
                                    None=>{
                                        return Err("failed-get_collection_from_collection_pool_as_mut".to_string());
                                    }
                                }
                            }
                        } else {
                            collections.insert(coll.cuid.clone(),coll);
                        }
                    },
                    Err(e)=>{
                        let error = format!("failed-process_collection=>{}",e);
                        return Err(error);
                    }
                }
            }
        }
    }

    //println!("collections : {:#?}",collections);

    //process
    match process::init(collections) {
        Ok(_)=>{
            println!(">>> process collections successfull");
        },
        Err(e)=>{
            println!("!!! failed-process_collection_vector=>{}",e);
        }
    }

    return Err("no_error".to_string());

}
