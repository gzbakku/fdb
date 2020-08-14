use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::{BASE_DIR,io,engine};
use json::{object,JsonValue};

lazy_static! {
    static ref COLLECTIONS:Mutex<HashMap<String,Collection>> = Mutex::new(HashMap::new());
}

#[derive(Debug,Clone)]
pub struct Collection {
    pub ccid:String,
    pub cuid:String,
    pub path:String,
    pub files:u64
}

pub fn ensure(cuid:&String,ccid:&String,path:&String) -> Result<(),String> {

    let base_dir:String;
    match BASE_DIR.lock() {
        Ok(bd)=>{
            base_dir = bd.path.clone();
        },
        Err(_)=>{
            return Err("failed-lock-BASE_DIR-mutex".to_string());
        }
    }

    let collection_path = format!("{}/collections/{}.fdbsc",base_dir,cuid);
    if io::ensure_file(&collection_path) {
        return Ok(());
    }

    let build = object!{
        "cuid" => cuid.to_string(),
        "ccid" => ccid.to_string(),
        "path" => path.to_string(),
        "files" => 0
    };

    match io::write(collection_path,build.dump()){
        Ok(_)=>{},
        Err(e)=>{
            let error = format!("failed-write_collection-ensure_collection=>{}",e);
            return Err(error);
        }
    }

    let for_pool = Collection {
        cuid:cuid.to_string(),
        ccid:ccid.to_string(),
        path:path.to_string(),
        files:0
    };

    match COLLECTIONS.lock() {
        Ok(mut pool)=>{
            pool.insert(for_pool.ccid.clone(),for_pool);
        },
        Err(_)=>{
            return Err("failed-lock_collections_mutex".to_string());
        }
    }

    return Ok(());

}

fn get_controller(cuid:String) -> Result<Collection,String> {
    match COLLECTIONS.lock() {
        Ok(mut pool)=>{
            if pool.contains_key(&cuid) {
                match pool.get(&cuid) {
                    Some(coll)=>{
                        return Ok(coll.clone());
                    },
                    None=>{
                        return Err("failed-extract_collection".to_string());
                    }
                }
            } else {
                match read_controller(&cuid){
                    Ok(coll)=>{
                        pool.insert(coll.cuid.clone(),coll.clone());
                        return Ok(coll);
                    },
                    Err(e)=>{
                        let error = format!("failed=>read_collection=>{}",e);
                        return Err(error);
                    }
                }
            }
        },
        Err(_)=>{
            return Err("failed-lock_collections_mutex".to_string());
        }
    }
}

pub fn read_controller(cuid:&String) -> Result<Collection,String> {

    let base_dir:String;
    match BASE_DIR.lock() {
        Ok(bd)=>{
            base_dir = bd.path.clone();
        },
        Err(_)=>{
            return Err("failed-lock-BASE_DIR-mutex".to_string());
        }
    }

    let body:JsonValue;
    let collection_path = format!("{}/collections/{}.fdbsc",&base_dir,&cuid);
    match io::read_collection_control(collection_path) {
        Ok(obj)=>{
            body = obj;
        },
        Err(e)=>{
            let error = format!("failed=>read_collection_control=>{}",e);
            return Err(error);
        }
    }

    if
        !body.has_key("ccid") ||
        !body.has_key("cuid") ||
        !body.has_key("path") ||
        !body.has_key("files")
    {
        return Err("invalid-collection_keys".to_string());
    }

    let mut build = Collection {
        ccid:String::new(),
        cuid:String::new(),
        path:String::new(),
        files:0
    };

    match body["ccid"].as_str() {
        Some(str)=>{
            build.ccid = str.to_string();
        },
        None=>{
            return Err("failed-extract-ccid".to_string());
        }
    }

    match body["cuid"].as_str() {
        Some(str)=>{
            build.cuid = str.to_string();
        },
        None=>{
            return Err("failed-extract-cuid".to_string());
        }
    }

    match body["path"].as_str() {
        Some(str)=>{
            build.path = str.to_string();
        },
        None=>{
            return Err("failed-extract-path".to_string());
        }
    }

    match body["files"].as_u64() {
        Some(int)=>{
            build.files = int;
        },
        None=>{
            return Err("failed-extract-files".to_string());
        }
    }

    return Ok(build);

}
