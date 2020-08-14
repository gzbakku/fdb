use crate::{FILE_LOCK,BASE_DIR};
use std::fs::read_to_string;
use std::{thread,time};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use postoffice::io::{check_path,new_file,ensure_dir,write,read,delete_file};
use std::collections::HashMap;

pub fn delete_file_from_disk(token:&Writer) -> Result<(),&'static str>{

    match verify(&token){
        Ok(v)=>{
            if !v{
                return Err("invalid-token-delete_file_from_disk-disk-writer");
            }
        },
        Err(e)=>{
            println!("{:?}",e);
            return Err("failed-verify-delete_file_from_disk-disk-writer");
        }
    }

    match delete_file(token.path.to_string()){
        Ok(_)=>{},
        Err(e)=>{
            println!("{:?}",e);
            return Err("failed-read_as_str-delete_file_from_disk-disk-writer");
        }
    }

    match unlock(&token){
        Ok(_)=>{
            return Ok(());
        },
        Err(_)=>{
            return Err("failed-unlock_file-delete_file_from_disk-disk-writer");
        }
    }

}

pub fn read_file(token:&Writer) -> Result<String,&'static str>{

    match verify(&token){
        Ok(v)=>{
            if !v{
                return Err("invalid-token-read_file-disk-writer");
            }
        },
        Err(e)=>{
            println!("{:?}",e);
            return Err("failed-verify-read_file-disk-writer");
        }
    }

    let raw:String;
    match read(&token.path){
        Ok(str)=>{
            raw = str;
        },
        Err(e)=>{
            println!("{:?}",e);
            return Err("failed-read_as_str-read_file-disk-writer");
        }
    }

    match unlock(&token){
        Ok(_)=>{
            return Ok(raw);
        },
        Err(_)=>{
            return Err("failed-unlock_file-read_file-disk-writer");
        }
    }

}

pub fn write_file(token:&Writer,v:String) -> Result<(),&'static str>{

    match verify(&token){
        Ok(v)=>{
            if !v{
                return Err("invalid-token-write_file-disk-writer");
            }
        },
        Err(e)=>{
            println!("{:?}",e);
            return Err("failed-verify-write_file-disk-writer");
        }
    }

    match write(token.path.clone(),v.into_bytes()){
        Ok(_)=>{},
        Err(e)=>{
            println!("{:?}",e);
            return Err("failed-write_to_file-write_file-disk-writer");
        }
    }

    match unlock(&token){
        Ok(_)=>{
            return Ok(());
        },
        Err(_)=>{
            return Err("failed-unlock_file-write_file-disk-writer");
        }
    }

}

pub fn verify(token:&Writer) -> Result<bool,&'static str> {
    match FILE_LOCK.lock(){
        Ok(lock)=>{
            //check if file lock exists
            if !lock.map.contains_key(&token.anchor){
                return Ok(false);
            } else {
                //check token
                match lock.map.get(&token.anchor){
                    Some(v)=>{
                        if v == &token.token{
                            return Ok(true);
                        } else {
                            return Ok(false);
                        }
                    },
                    None=>{
                        return Ok(false);
                    }
                }
            }
        },
        Err(_)=>{
            return Err("failed-open_file_locker-verify-disk-writer");
        }
    }
}

pub fn unlock(token:&Writer) -> Result<(),&'static str>{
    match FILE_LOCK.lock(){
        Ok(mut lock)=>{
            match lock.map.remove(&token.anchor){
                Some(_)=>{},
                None=>{}
            }
            return Ok(());
        },
        Err(_)=>{
            return Err("failed-open_file_locker-get_token-disk-writer");
        }
    }
}

#[derive(Debug,Clone)]
pub struct Writer{
    pub path:String,
    pub anchor:String,
    pub token:String
}

pub fn get_token(file_name:&String,file_type:&String) -> Result<Writer,&'static str> {

    let file_anchor = format!("{}_{}",&file_type,&file_name);
    let base_dir:String;
    match BASE_DIR.lock(){
        Ok(lock)=>{
            base_dir = lock.path.clone();
        },
        Err(_)=>{
            return Err("failed-flush_collection-get_token-disk-writer");
        }
    }
    let file_dir = format!("{}/{}",base_dir,&file_type);
    let file_path = format!("{}/{}",file_dir,&file_name);

    let token:String;
    loop{
        match FILE_LOCK.lock(){
            Ok(mut lock)=>{
                //check if file lock exists
                if lock.map.contains_key(&file_anchor){
                    let sleep = time::Duration::from_millis(100);
                    thread::sleep(sleep);
                } else {
                    //generate token
                    let token_str: String = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(30)
                    .collect();
                    //insert token
                    match lock.map.insert(file_anchor.clone(),token_str.clone()){
                        Some(_)=>{},
                        None=>{}
                    }
                    //adopt token
                    token = token_str;
                    break;
                }
            },
            Err(_)=>{
                return Err("failed-open_file_locker-get_token-disk-writer");
            }
        }
    }

    if !ensure_dir(&file_dir){
        return Err("failed-ensure_dir-get_token-disk-writer");
    }

    if !check_path(&file_path){
        if !new_file(&file_path){
            return Err("failed-ensure_file-get_token-disk-writer");
        }
    }

    return Ok(Writer{
        path:file_path,
        anchor:file_anchor,
        token:token
    });

}

pub fn get_base_dir() -> Result<String,&'static str>{
    match BASE_DIR.lock(){
        Ok(lock)=>{
            return Ok(lock.path.clone());
        },
        Err(_)=>{
            return Err("failed-lock_base_dir-get_base_dir-disk-writer");
        }
    }
}

#[derive(Debug,Clone)]
pub struct FdbMap{
    pub map:HashMap<u128,String>
}

pub fn parse_map(w:&Writer) -> Result<FdbMap,&'static str>{

    let raw:String;
    match read_to_string(&w.path){
        Ok(str)=>{
            raw = str;
        },
        Err(_)=>{
            return Err("failed-read_raw_file-parse_map-disk-writer");
        }
    }

    if raw.len() == 0{
        return Ok(FdbMap{
            map:HashMap::new()
        });
    }

    match parse_map_str(&raw){
        Ok(map)=>{
            return Ok(map);
        },
        Err(e)=>{
            println!("{:?}",e);
            return Err("failed-parse_map_str-parse_map-disk-writer");
        }
    }


}

pub fn parse_map_str(raw:&String) -> Result<FdbMap,&'static str> {

    let lines = raw.split("||--||").collect::<Vec<&str>>();
    let mut map:HashMap<u128,String> = HashMap::new();

    for line in lines{
        if line.len() > 0 {
            if !line.contains("++==++"){
                return Err("invalid_data-parse_lines-parse_map_str-disk-writer");
            } else {
                let line_vec = line.split("++==++").collect::<Vec<&str>>();
                if line_vec.len() != 2{
                    return Err("invalid_data-seprator_error-parse_lines-parse_map_str-disk-writer");
                }
                match line_vec[0].parse::<u128>(){
                    Ok(v)=>{
                        match map.insert(v,line_vec[1].to_string()){
                            Some(_)=>{},
                            None=>{}
                        }
                    },
                    Err(_)=>{
                        return Err("failed-parse_index-parse_map_str-disk-writer");
                    }
                }
            }
        }
    }

    return Ok(FdbMap{
        map:map
    });

}
