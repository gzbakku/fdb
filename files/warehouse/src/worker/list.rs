use crate::formats::Act;
use crate::worker::disk::get_base_dir;
use json::JsonValue;
use std::fs;

pub fn init(act:Act) -> Result<JsonValue,&'static str> {

    let base_dir:String;
    match get_base_dir(){
        Ok(str)=>{
            base_dir = str;
        },
        Err(_)=>{
            return Err("failed-get_base_dir-init-list-worker");
        }
    }

    let this_dir = format!("{}/{}",base_dir,act.file_type);
    let reader:fs::ReadDir;
    match fs::read_dir(this_dir){
        Ok(r)=>{
            reader = r;
        },
        Err(_)=>{
            return Err("failed-read_dir-init-list-worker");
        }
    }

    let mut files = JsonValue::new_array();
    for entry in reader{
        match entry{
            Ok(dir)=>{
                let file_name = dir.file_name();
                match file_name.to_str(){
                    Some(v)=>{
                        match files.push(v.to_string()){
                            Ok(_)=>{},
                            Err(_)=>{
                                return Err("failed-push_into_files-init-list-worker");
                            }
                        }
                    },
                    None=>{
                        return Err("failed-parse_file_name-init-list-worker");
                    }
                }
            },
            Err(_)=>{
                return Err("failed-parse_entry_to_path-init-list-worker");
            }
        }
    }

    return Ok(files);

}
