use crate::formats::Act;
use json::JsonValue;
use crate::find;

pub fn init(act:&Act) -> Result<JsonValue,&'static str> {

    match find::init(act.file_name.clone(), act.file_type.clone()){
        Ok(found)=>{
            let mut build = JsonValue::new_object();
            build.insert("found",found).unwrap();
            return Ok(build);
        },
        Err(_)=>{
            return Err("failed-find_file_in_warehouse");
        }
    }



}
