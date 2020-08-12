// #[macro_use]
// extern crate lazy_static;

use std::thread;

mod client;

mod common;

use client::{start_connection,send_message,get_test_message};

use std::time::Duration;

fn main(){
    if false {
        client();
    }

    for _ in 0..1 {
        commons();
    }
}

fn commons(){

    let uid = common::uid(8);

    println!("uid : {:?}",&uid);

    let hash_md5 = common::hash::md5(&uid);

    println!("md5 : {:?}",hash_md5);

    let hash_sha256 = common::hash::sha256(&uid);

    println!("sha256 : {:?}",hash_sha256.len());

}

fn client(){

    let key = "8cfb30b34977529853bbe46afdbbd5ae".to_string();
    let connection_id = client::get_random_connection_id();
    let addr = "127.0.0.1:5200".to_string();

    match start_connection(&connection_id,addr,key) {
        Ok(_)=>{
            println!("connection establishged");
        },
        Err(_)=>{
            client::common::error("failed start connection");
        }
    }

    let message = get_test_message(8);
    match send_message(&connection_id, message.clone(), false) {
        Ok(response)=>{
            if response.message.contains(&message) {
                println!("request successfull");
                println!("response final : {:#?}",response);
            } else {
                println!("response final : {:?}",response);
            }
        },
        Err(_)=>{
            client::common::error("request-failed");
        }
    }

    //thread::sleep(Duration::from_millis(1000));

}
