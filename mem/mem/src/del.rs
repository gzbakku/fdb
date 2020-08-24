use crate::formats::Act;
use json::JsonValue;
use crate::{book,find};
use postoffice::client::channel::send_to_member;
use futures::executor::block_on;
use postoffice::resp::is_successfull;

pub fn init(act:&Act) -> Result<(),&'static str> {

    match book::check(act.index.clone()){
        Ok(a)=>{
            if a{
                if a{
                    match remove(&act.index){
                        Ok(_)=>{
                            return Ok(());
                        },
                        Err(_)=>{
                            return Err("failed-fetch_item-init-delete");
                        }
                    }
                } else {
                    return Ok(());
                }
            }
        },
        Err(_)=>{
            return Err("failed-check_local_book-init-delete");
        }
    }



    match find::init(&act.index){
        Ok(found)=>{
            if found{
                match remove(&act.index){
                    Ok(_)=>{
                        return Ok(());
                    },
                    Err(_)=>{
                        return Err("failed-fetch_item-init-delete");
                    }
                }
            } else {
                return Ok(());
            }
        },
        Err(_)=>{
            return Err("failed-parse_brodcast-init-delete");
        }
    }

}

fn remove(index:&String) -> Result<(),&'static str>{

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
    req.insert("type","delete").unwrap();
    let mut data = JsonValue::new_object();
    data.insert("index",index.clone()).unwrap();
    req.insert("data",data).unwrap();

    match block_on(send_to_member(&"memBank".to_string(), &member_name, &req, false)){
        Ok(resp)=>{
            if !is_successfull(&resp.data){
                match book::remove(index.clone()){
                    Ok(_)=>{
                        return Ok(());
                    },
                    Err(_)=>{
                        return Err("failed-remove_from_book-fetch-init-get");
                    }
                }
            } else {
                return Err("failed-remove_from_member-fetch-init-get");
            }
        },
        Err(_)=>{
            return Err("failed-fetch_item_from_member-fetch-init-get");
        }
    }

}
