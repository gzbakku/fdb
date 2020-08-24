use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;

lazy_static! {
    static ref BOOK : Mutex<HashMap<String,String>> = Mutex::new(HashMap::new());
}

pub fn add(index:String,value:String) -> Result<(),&'static str>{
    match BOOK.lock(){
        Ok(mut book)=>{
            match book.insert(index,value){
                Some(_)=>{},
                None=>{}
            }
            return Ok(());
        },
        Err(_)=>{
            return Err("failed-lock_book-add-location");
        }
    }
}

pub fn check(index:String) -> Result<bool,&'static str>{
    match BOOK.lock(){
        Ok(book)=>{
            if book.contains_key(&index){
                return Ok(true);
            } else {
                return Ok(false);
            }
        },
        Err(_)=>{
            return Err("failed-lock_book-check-location");
        }
    }
}

pub fn get(index:String) -> Result<String,&'static str>{
    match BOOK.lock(){
        Ok(book)=>{
            match book.get(&index){
                Some(v)=>{
                    return Ok(v.to_string());
                },
                None=>{
                    return Err("failed-get_value-get-location");
                }
            }
        },
        Err(_)=>{
            return Err("failed-lock_book-get-location");
        }
    }
}

pub fn remove(index:String) -> Result<(),&'static str>{
    match BOOK.lock(){
        Ok(mut book)=>{
            match book.remove(&index){
                Some(_)=>{
                    return Ok(());
                },
                None=>{
                    return Err("failed-get_value-remove-location");
                }
            }
        },
        Err(_)=>{
            return Err("failed-lock_book-remove-location");
        }
    }
}
