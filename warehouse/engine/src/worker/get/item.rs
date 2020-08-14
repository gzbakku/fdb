use crate::formats::Act;
use crate::worker::disk::{get_token,Writer,read_file,FdbMap,parse_map_str};
use json::JsonValue;

pub fn init(act:Act) -> Result<JsonValue,&'static str> {

    let token:Writer;
    match get_token(&act.file_name,&act.file_type){
        Ok(t)=>{
            token = t;
        },
        Err(_)=>{
            return Err("failed-get_file_lock-init-item-get-worker");
        }
    }

    let raw:String;
    match read_file(&token){
        Ok(str)=>{
            raw = str;
        },
        Err(_)=>{
            return Err("failed-read_file-init-item-get-worker");
        }
    }

    let map:FdbMap;
    match parse_map_str(&raw){
        Ok(m)=>{
            map = m;
        },
        Err(_)=>{
            return Err("failed-parse_map_str-init-item-get-worker");
        }
    }

    match map.map.get(&act.item_index){
        Some(v)=>{
            let mut docs = JsonValue::new_object();
            docs.insert(&act.item_index.to_string(),v.to_string()).unwrap();
            let mut resp = JsonValue::new_object();
            resp.insert("docs",docs).unwrap();
            return Ok(resp);
        },
        None=>{
            let docs = JsonValue::new_object();
            let mut resp = JsonValue::new_object();
            resp.insert("docs",docs).unwrap();
            return Ok(resp);
        }
    }

}
