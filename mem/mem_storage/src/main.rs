use postoffice::{server,resp};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;

struct Channel{
    map:HashMap<String,String>
}

lazy_static! {
    static ref Channels : Mutex<HashMap<String,Channel>> = Mutex::new(HashMap::new());
}

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

    return Ok(resp::ok(r));

}
