use postoffice::client;
use json::JsonValue;
use postoffice::client::common;

mod add;

fn main() {

    let key = "4db2e31021831bcc09af0347947563a8".to_string();
    let connection_id = client::get_random_connection_id();
    let addr = "127.0.0.1:5205".to_string();

    match client::start_connection(&connection_id,addr,key) {
        Ok(_)=>{
            println!("connection established");
        },
        Err(_)=>{
            common::error("failed start connection");
        }
    }

    add::init(&connection_id);

}
