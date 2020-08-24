use crate::formats::Act;
use json::JsonValue;
use crate::{book,find};

pub fn init(act:&Act) -> Result<JsonValue,&'static str> {

    match book::check(act.index.clone()){
        Ok(a)=>{
            if a{
                let mut data = JsonValue::new_object();
                data.insert("exists",true).unwrap();
                return Ok(data);
            }
        },
        Err(_)=>{
            return Err("failed-check_local_book-init-check");
        }
    }

    match find::init(&act.index){
        Ok(found)=>{
            let mut data = JsonValue::new_object();
            data.insert("exists",found).unwrap();
            return Ok(data);
        },
        Err(_)=>{
            return Err("failed-parse_brodcast-init-check");
        }
    }

}
