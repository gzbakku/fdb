use postoffice::{server,resp,check};
use json::JsonValue;
use postoffice::check::{Field,Format};

mod path;
mod collector;
mod collections;
mod validate;

fn main() {

    let key = "8cfb30b34977529853bbe46afdbbd5ae".to_string();
    let address = String::from("127.0.0.1:5200");

    let cwd = collector::io::cwd();
    let base_dir = format!("{}/instance",cwd);
    if !collector::io::ensure_dir(&base_dir) {
        println!("!!! failed-snsure_base_dir-for_collector");
        return;
    }

    match collections::process_collections(&base_dir) {
        Ok(_)=>{},
        Err(_)=>{
            println!("failed init collector");
            return;
        }
    }

    server::init(address,key,handler,auth);

}


fn auth(_token:server::auth::Token) -> bool {
    true
}

fn handler(req: server::Request) -> Result<server::Response,String> {

    let body:JsonValue;
    match validate::request(req.clone()) {
        Ok(parsed)=>{
            body = parsed;
        },
        Err(e)=>{
            let error = format!("check request failed error : {:?}",e);
            return Ok(resp::error(req,error));
        }
    }

    if true {
        insert(&body);
    }

    return Ok(resp::ok(req));

}

fn insert(user:&JsonValue){

    match collector::insert(&user.dump()) {
        Ok(_)=>{
            //println!(">>> insert successfull");
        },
        Err(e)=>{
            println!("!!! failed-insert=>{}",e);
        }
    }

}
