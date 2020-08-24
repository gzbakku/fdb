use postoffice::client;
use json::JsonValue;
use postoffice::client::{common,channel};

mod add;
mod check;
mod get;
mod del;

fn main() {

    let key = "2dc6bb40e73417bba878d3c8e3e08780".to_string();
    // let connection_id = client::get_random_connection_id();
    let addr_one = "127.0.0.1:5611".to_string();
    let addr_two = "127.0.0.1:5612".to_string();

    if true{
        match channel::add_member(&"mem".to_string(), &"one".to_string(), &addr_one, &key){
            Ok(_)=>{
                println!("add_member-successfull");
            },
            Err(e)=>{
                println!("failed-add_memeber_to_channel-{:?}",e);
                return;
            }
        }
    }

    if false{
        match channel::add_member(&"mem".to_string(), &"two".to_string(), &addr_two, &key){
            Ok(_)=>{
                println!("add_member-successfull");
            },
            Err(e)=>{
                println!("failed-add_memeber_to_channel-{:?}",e);
                return;
            }
        }
    }

    // match client::start_connection(&connection_id,addr,key) {
    //     Ok(_)=>{
    //         println!("connection established");
    //     },
    //     Err(_)=>{
    //         common::error("failed start connection");
    //     }
    // }

    if true{
        add::init(&"mem".to_string());
        println!("\n\n\n");
    }
    if true{
        check::init(&"mem".to_string());
        println!("\n\n\n");
    }
    if true{
        get::init(&"mem".to_string());
        println!("\n\n\n");
    }
    if true{
        del::init(&"mem".to_string());
        println!("\n\n\n");
    }
    if true{
        get::init(&"mem".to_string());
        println!("\n\n\n");
    }

}
