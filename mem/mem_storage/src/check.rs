use json::JsonValue;
use crate::book;

pub fn init(index:&String) -> Result<JsonValue,&'static str> {
    match book::check(index.to_string()){
        Ok(bool)=>{
            let mut build = JsonValue::new_object();
            build.insert("exists",bool).unwrap();
            return Ok(build);
        },
        Err(_)=>{
            return Err("failed-get_from_book");
        }
    }
}
