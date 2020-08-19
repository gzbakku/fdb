use crate::worker::collectify::FdbFile;
use crate::worker::disk::{FdbMap,parse_map_str};
use std::collections::HashMap;

pub fn init(file:&FdbFile,map:&mut FdbMap) -> Result<String,&'static str>{

    for act in &file.acts{
        if act.func == "add_item"{
            match map.map.insert(act.item_index.clone(),act.item_value.clone()){
                Some(_)=>{},
                None=>{}
            }
        } else if act.func == "delete_item"{
            match map.map.remove(&act.item_index){
                Some(_)=>{},
                None=>{}
            }
        } else if act.func == "add_file"{
            match parse_map_str(&act.item_value){
                Ok(m)=>{
                    map.map = m.map;
                },
                Err(_)=>{
                    println!("failed-parse_map_str-init-actify-writer");
                }
            }
        } else if act.func == "delete_file"{
            map.map = HashMap::new();
        }
    }

    match parse_map_to_str(&map.map){
        Ok(str)=>{
            return Ok(str);
        },
        Err(e)=>{
            print!("{:?}",e);
            return Err("failed-parse_map_to_str-init-actify-writer");
        }
    }

}

fn parse_map_to_str(m:&HashMap<String,String>) -> Result<String,&'static str>{

    let mut collect = String::new();

    for key in m.keys(){
        match m.get(&key.to_string()){
            Some(v)=>{
                let this_line = format!("{}++==++{}||--||",key,v);
                collect = format!("{}{}",collect,this_line);
            },
            None=>{
                return Err("failed-get_val_from_map-parse_map_to_str-actify-writer");
            }
        }
    }

    return Ok(collect);

}
