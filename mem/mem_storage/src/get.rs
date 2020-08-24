use json::JsonValue;
use crate::book;

pub fn init(index:&String) -> Result<JsonValue,&'static str> {

    let value:String;
    match book::get(index.to_string()){
        Ok(v)=>{
            value = v;
        },
        Err(_)=>{
            return Err("failed-get_from_book");
        }
    }

    let mut build = JsonValue::new_object();
    match build.insert(index,value){
        Ok(_)=>{},
        Err(_)=>{
            return Err("failed-get_from_book");
        }
    }

    return Ok(build);

}
