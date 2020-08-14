use crate::collector::{CONTROL,BASE_DIR,io,COLLECTIONS};
use std::fs::File;
use std::io::{BufReader};

#[derive(Debug,Clone)]
pub struct Collection {
    pub name:String,
    pub path:String,
    pub base_dir:String,
    pub data:Vec<String>
}

pub fn set_collection_action(action:&str,name:String) -> Result<(),String>{
    match COLLECTIONS.lock() {
        Ok(mut collections)=>{
            if action == "flush" {
                collections.flush.push(name);
            } else {
                return Err("invalid-action".to_string());
            }
            println!("--- do not close until collection edit commit ---");
            return Ok(());
        },
        Err(e)=>{
            let error = format!("failed-lock_collection=>{}",e);
            return Err(error);
        }
    }
}

#[allow(dead_code)]
impl Collection {
    #[allow(dead_code)]
    pub fn flush(self:&mut Self) -> Result<(),String> {
        if true {
            match io::delete_file(self.path.clone()) {
                Ok(_)=>{},
                Err(e)=>{
                    let error = format!("failed-flush_collection=>{}",e);
                    return Err(error);
                }
            }
        }
        if true {
            match CONTROL.lock() {
                Ok(mut control)=>{
                        if control.finished.len() > 0{
                            control.finished.remove(0);
                        }
                },
                Err(_)=>{
                    return Err("failed-lock_ACTIVE_mutex".to_string());
                }
            }
        }
        match set_collection_action("flush",self.name.clone()) {
            Ok(_)=>{
                return Ok(());
            },
            Err(e)=>{
                let error = format!("failed-set_collection_action=>{}",e);
                return Err(error);
            }
        }
    }
    #[allow(dead_code)]
    pub fn reader(self:&mut Self) -> Result<BufReader<File>,String> {
        match File::open(&self.path.clone()) {
            Ok(reader)=>{
                let lines = BufReader::new(reader);
                return Ok(lines);
            },
            Err(e)=>{
                let error = format!("failed-open_file=>{}",e);
                return Err(error);
            }
        }
    }
}

#[allow(dead_code)]
pub fn get(read:bool) -> Result<Collection,String> {

    let base_dir:String;
    match BASE_DIR.lock() {
        Ok(base)=>{
            base_dir = base.path.clone();
        },
        Err(_)=>{
            return Err("failed-lock_BaseDir_mutex".to_string());
        }
    }

    let collection_name:String;
    match CONTROL.lock() {
        Ok(control)=>{
                if control.finished.len() == 0{
                    return Err("no_collections_found".to_string());
                }
                collection_name = control.finished[0].to_string();
        },
        Err(_)=>{
            return Err("failed-lock_ACTIVE_mutex".to_string());
        }
    }

    let path = format!("{}/{}.fdbcs",&base_dir,&collection_name);

    if !io::check_path(&path){
        match CONTROL.lock() {
            Ok(mut control)=>{
                    if control.finished.len() > 0{
                        control.finished.remove(0);
                    }
            },
            Err(_)=>{
                return Err("failed-lock_ACTIVE_mutex".to_string());
            }
        }
        match set_collection_action("flush",collection_name) {
            Ok(_)=>{
                let error = format!("failed-absent_collection_file-reseting_collection");
                return Err(error);
            },
            Err(e)=>{
                let error = format!("failed-absent_collection_file_reset_with_flush=>{}",e);
                return Err(error);
            }
        }
    }

    let mut collect = Vec::new();
    if read {
        match io::read(&path) {
            Ok(data)=>{
                let hold = data.split("\n").collect::<Vec<&str>>();
                for item in hold {
                    collect.push(item.to_string());
                }
            },
            Err(e)=>{
                //ensure file if not flush this File
                let error = format!("failed-read_collection_file=>{}=>{}",&path,e);
                return Err(error);
            }
        }
    }

    let collection = Collection {
        name:collection_name,
        path:path,
        base_dir:base_dir,
        data:collect
    };

    return Ok(collection);

}
