use crate::formats::Act;
use json::JsonValue;
use crate::{find,book};
use postoffice::client::channel::send_to_member;
use postoffice::resp::{is_successfull,with_data};
use futures::executor::block_on;

pub fn init(act:&Act) -> Result<JsonValue,&'static str> {

    //check it in local book
    let file_anchor = format!("{}_{}",&act.file_name.clone(),&act.file_type.clone());
    match book_check(&file_anchor,&act){
        Ok(result)=>{
            return Ok(result);
        },
        Err(_)=>{
            // println!("!!! failed-book_check-init-get_range-worker=>{:?}",e);
        }
    }

    match find::init(act.file_name.clone(), act.file_type.clone()){
        Ok(v)=>{
            if !v{
                return Ok(no_found_response());
            }
        },
        Err(_)=>{
            return Err("failed-find_file_in_warehouse");
        }
    }

    match book_check(&file_anchor,&act){
        Ok(result)=>{
            return Ok(result);
        },
        Err(_)=>{
            // println!("!!! failed-book_check-after_find-init-get_range-worker=>{:?}",e);
            return Err("failed-book_check-after_find-init-get_range-worker");
        }
    }

}

fn book_check(file_anchor:&String,act:&Act) -> Result<JsonValue,&'static str>{
    match book::check(file_anchor.clone()) {
        Ok(found)=>{
            if found{
                match book::get(file_anchor.clone()){
                    Ok(member_name)=>{
                        match call_item(&act, &member_name){
                            Ok(data)=>{
                                return Ok(data);
                            },
                            Err(_)=>{
                                // println!("!!! failed-call_member_for_item-init-get_ranges-worker => {:?}",e);
                                return Err("failed-call_member_for_item-init-get_ranges-worker");
                            }
                        }
                    },
                    Err(_)=>{
                        return Err("failed-get_book_item-init-get_ranges-worker");
                    }
                }
            } else {
                return Err("not_found-get_book_item-init-get_ranges-worker");
            }
        },
        Err(_)=>{
            return Err("failed-check_book-init-get_ranges-worker");
        }
    }
}

fn call_item(act:&Act,member_name:&String) -> Result<JsonValue,&'static str>{
    let mut req = JsonValue::new_object();
    req.insert("type","get_range").unwrap();
    let mut data = JsonValue::new_object();
    data.insert("file_name",act.file_name.clone()).unwrap();
    data.insert("file_type",act.file_type.clone()).unwrap();
    data.insert("start_index",act.start_index.to_string()).unwrap();
    data.insert("end_index",act.end_index.to_string()).unwrap();
    req.insert("data",data).unwrap();
    match block_on(send_to_member(&"warehouse".to_string(), member_name, &req, false)){
        Ok(resp)=>{
            // println!("{:?}",resp);
            if !is_successfull(&resp.data) || !with_data(&resp.data){
                return Err("failed-process_request-call-init-get_ranges-worker");
            }
            return Ok(resp.data["data"].clone());
        },
        Err(_)=>{
            return Err("failed-send_request-call-init-get_ranges-worker");
        }
    }
}

fn no_found_response() -> JsonValue{
    let mut build = JsonValue::new_object();
    build.insert("docs",JsonValue::new_object()).unwrap();
    return build;
}
