use postoffice::{server,resp};
use lazy_static::lazy_static;
use std::sync::Mutex;

mod formats;
mod storage;

fn main() {
    let key = "4db2e31021831bcc09af0347947563a8".to_string();
    let address = String::from("127.0.0.1:5205");
    server::init(address,key,handler,auth);
}

fn auth(_:server::auth::Token) -> bool {
    //println!("token : {:?}",token);
    return true;
}

fn handler(r:server::Request) -> Result<server::Response,String>{

    let act:formats::Act;
    match formats::parse_request(&r.data){
        Ok(a)=>{
            act = a;
        },
        Err(_)=>{
            return Ok(resp::error(r, "failed-parse-request".to_string()));
        }
    }

    println!("{:?}",act);

    return Ok(resp::ok(r));

}
