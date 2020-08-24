use crate::formats::Act;
use json::JsonValue;
use crate::{book,find};
use postoffice::client::channel::send_to_member;
use futures::executor::block_on;
use postoffice::resp::{is_successfull,with_data};

pub fn init(act:&Act) -> Result<JsonValue,&'static str> {

    match book::check(act.index.clone()){
        Ok(a)=>{
            if a{
                if a{
                    match fetch(&act.index){
                        Ok(data)=>{
                            return Ok(data);
                        },
                        Err(_)=>{
                            return Err("failed-fetch_item-init-get");
                        }
                    }
                } else {
                    return Ok(not_found());
                }
            }
        },
        Err(_)=>{
            return Err("failed-check_local_book-init-get");
        }
    }



    match find::init(&act.index){
        Ok(found)=>{
            if found{
                match fetch(&act.index){
                    Ok(data)=>{
                        return Ok(data);
                    },
                    Err(_)=>{
                        return Err("failed-fetch_item-init-get");
                    }
                }
            } else {
                return Ok(not_found());
            }
        },
        Err(_)=>{
            return Err("failed-parse_brodcast-init-get");
        }
    }

}

fn fetch(index:&String) -> Result<JsonValue,&'static str>{

    let member_name:String;
    match book::get(index.clone()){
        Ok(m)=>{
            member_name = m;
        },
        Err(_)=>{
            return Err("failed-get_member_from_book-fetch-init-get");
        }
    }

    let mut req = JsonValue::new_object();
    req.insert("type","get").unwrap();
    let mut data = JsonValue::new_object();
    data.insert("index",index.clone()).unwrap();
    req.insert("data",data).unwrap();

    match block_on(send_to_member(&"memBank".to_string(), &member_name, &req, false)){
        Ok(resp)=>{
            if !is_successfull(&resp.data) || !with_data(&resp.data){
                return Ok(not_found());
            } else {
                return Ok(resp.data["data"].clone());
            }
        },
        Err(_)=>{
            return Err("failed-fetch_item_from_member-fetch-init-get");
        }
    }

}

fn not_found() -> JsonValue{
    let mut data = JsonValue::new_object();
    data.insert("docs",JsonValue::new_object()).unwrap();
    return data;
}
