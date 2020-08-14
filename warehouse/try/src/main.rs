use postoffice::client;
use postoffice::client::{start_connection,common};

mod add;
mod get;
mod del;
mod list;

fn main() {

    let key = "8cfb30b34977529853bbe46afdbbd5ae".to_string();
    let connection_id = client::get_random_connection_id();
    let addr = "127.0.0.1:5200".to_string();

    match start_connection(&connection_id,addr,key) {
        Ok(_)=>{
            println!("connection established");
        },
        Err(_)=>{
            common::error("failed start connection");
        }
    }

    if true{
        add::init(&connection_id);
    }

    if false{
        get::init(&connection_id);
    }

    if false{
        del::init(&connection_id)
    }

    if false{
        list::init(&connection_id);
    }

}
