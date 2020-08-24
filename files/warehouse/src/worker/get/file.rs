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

    let mut docs = JsonValue::new_object();
    for key in map.map.keys(){
        match map.map.get(&key.to_string()){
            Some(v)=>{
                match docs.insert(&key.to_string(),v.to_string()){
                    Ok(_)=>{},
                    Err(_)=>{}
                }
            },
            None=>{
                return Err("not_found");
            }
        }
    }

    let mut resp = JsonValue::new_object();
    resp.insert("docs",docs).unwrap();
    return Ok(resp);

}
