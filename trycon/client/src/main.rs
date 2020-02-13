use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use std::time::{Instant};

mod client;

use client::{start_connection,common,send_message,get_test_message};

fn main(){

    let initial = Instant::now();

    let mut collect = Vec::new();

    for _ in 0..100 {
        collect.push(thread::spawn(move || {
            start();
        }));
    }

    for i in collect {
        match i.join() {
            Ok(_)=>{},
            Err(_)=>{}
        }
    }

    let duration = initial.elapsed();

    println!("request completion time : {:?}", duration);

}

fn start() {

    let key = "8cfb30b34977529853bbe46afdbbd5ae".to_string();
    let connection_id = client::get_random_connection_id();
    let addr = "127.0.0.1:5200".to_string();

    match start_connection(&connection_id,addr,key) {
        Ok(_)=>{
            //println!("connection establishged");
        },
        Err(_)=>{
            common::error("failed start connection");
        }
    }

    if false {
        thread::sleep(Duration::from_millis(100000));
        return;
    }

    let mut threads = Vec::new();

    for _ in 0..10 {
        threads.push(send_test_message(&connection_id));
    }

    for t in threads {
        match t.join() {
            Ok(_)=>{},
            Err(_)=>{}
        }
    }

    //thread::sleep(Duration::from_millis(5000));

}

fn send_test_message(id:&String) -> JoinHandle<()> {
    let connection_id = id.clone();
    let handle = thread::spawn(move || {
        let message = get_test_message(800);
        match send_message(&connection_id, message.clone(), false) {
            Ok(response)=>{
                //println!("request successfull");
                //println!("response final : {:#?}",response);
            },
            Err(_)=>{
                common::error("request-failed");
            }
        }
    });
    return handle;
}
