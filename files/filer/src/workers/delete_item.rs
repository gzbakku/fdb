use crate::formats::Act;
use json::JsonValue;
use crate::{book,find};
use postoffice::client::channel::{send_to_member,send};
use postoffice::resp::is_successfull;
use postoffice::common::log_check;
use futures::executor::block_on;

pub fn init(act:&Act) -> Result<(),&'static str> {

    let log = false;

    log_check("called delete_item",&log);

    let file_anchor = format!("{}_{}",act.file_name.clone(),act.file_type.clone());

    //build request
    let mut req = JsonValue::new_object();
    req.insert("type","delete_item").unwrap();
    let mut data = JsonValue::new_object();
    data.insert("file_name",act.file_name.clone()).unwrap();
    data.insert("file_type",act.file_type.clone()).unwrap();
    data.insert("item_index",act.item_index.clone()).unwrap();
    req.insert("data",data).unwrap();

    //check the book
    match book_check(&file_anchor, &req){
        Ok(found)=>{
            if found{
                log_check("added_item-ref_bookcheck",&log);
                return Ok(());
            }
        },
        Err(_)=>{
            // println!("!!! failed-book_check-basic => {:?}",e);
            return Err("failed-book_check-basic");
        }
    }

    //find the file
    match find::init(act.file_name.clone(), act.file_type.clone()){
        Ok(v)=>{
            if v{
                log_check("found_holder_warehouse",&log);
                match book_check(&file_anchor, &req){
                    Ok(_)=>{
                        log_check("added_item-after_find_bookcheck",&log);
                        return Ok(());
                    },
                    Err(_)=>{
                        return Err("failed-book_check-after_find");
                    }
                }
            }
        },
        Err(_)=>{
            return Err("failed-find_file_in_warehouse");
        }
    }

    //make a new file in a random warehouse
    match send_to_random_member(&req,&file_anchor){
        Ok(_)=>{
            log_check("added_item-random_warehouse",&log);
            return Ok(());
        },
        Err(_)=>{
            return Err("failed-send_to_random_member-init-delete_item");
        }
    }
}

fn book_check(file_anchor:&String,message:&JsonValue) -> Result<bool,&'static str>{
    let log = false;
    match book::check(file_anchor.clone()){
        Ok(f)=>{
            log_check("check_completed-book_check",&log);
            if f{
                log_check("item_found-book_check",&log);
                match book::get(file_anchor.clone()){
                    Ok(warehouse_name)=>{
                        log_check("got_item-book_check",&log);
                        match send_to_this_member(message,warehouse_name.clone()){
                            Ok(_)=>{
                                return Ok(true);
                            },
                            Err(_)=>{
                                return Err("failed-send_to_this_member");
                            }
                        }
                    },
                    Err(_)=>{
                        return Err("failed-get_warehouse_name_from_book");
                    }
                }
            } else {
                log_check("item_not_found-book_check",&log);
                return Ok(false);
            }
        },
        Err(_)=>{
            return Err("failed-open_book_to_check");
        }
    }
}

fn send_to_this_member(message:&JsonValue,member_name:String) -> Result<(),&'static str>{
    match block_on(send_to_member(&"warehouse".to_string(),&member_name,message,false)){
        Ok(resp)=>{
            if is_successfull(&resp.data){
                return Ok(());
            } else {
                return Err("failed-process_request-send_to_this_member-init-delete_item-worker");
            }
        },
        Err(_)=>{
            return Err("failed-send_request-send_to_this_member-init-delete_item-worker");
        }
    }
}

fn send_to_random_member(message:&JsonValue,file_anchor:&String) -> Result<(),&'static str>{
    match block_on(send(&"warehouse".to_string(),message,false)){
        Ok(resp)=>{
            if is_successfull(&resp.data){
                match book::add(file_anchor.clone(), resp.name.clone()){
                    Ok(_)=>{
                        return Ok(());
                    },
                    Err(_)=>{
                        println!("!!! failed-add_member_name_for_file_anchor_to_book-send_to_random_member-init-delete_item-worker");
                        return Ok(());
                    }
                }
            } else {
                return Err("failed-process_request-send_to_random_member");
            }
        },
        Err(_)=>{
            return Err("failed-send_request-send_to_random_member");
        }
    }
}
