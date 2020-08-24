use crate::BASE_DIR;
use crate::formats::Act;
// use crate::worker::disk::{get_token,Writer,read_file,FdbMap,parse_map_str};
use json::JsonValue;
use postoffice::io::check_path;

pub fn init(act:Act) -> Result<JsonValue,&'static str> {

    let file_path:String;
    match BASE_DIR.lock(){
        Ok(base_dir)=>{
            let file_dir = format!("{}/{}",base_dir.path.clone(),&act.file_type.clone());
            file_path = format!("{}/{}",file_dir,&act.file_name.clone());
        },
        Err(_)=>{
            return Err("failed-flush_collection-get_token-disk-writer");
        }
    }

    let mut found = false;
    if check_path(&file_path){
        found = true;
    }

    let mut resp = JsonValue::new_object();
    resp.insert("found",found).unwrap();
    return Ok(resp);

}
