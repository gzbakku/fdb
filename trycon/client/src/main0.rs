use std::net::TcpStream;
use std::io::{Read,Write};
use base64::{encode};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::time::Duration;
use std::thread;

fn error(e:&str){
    println!("!!! {:?}",e);
}

fn log(m:&str){
    println!(">>> {:?}",m);
}

fn main(){

    for _ in 0..1 {
        start();
    }

}

fn start() {

    log("hey its client here");

    let mut stream;
    match TcpStream::connect("127.0.0.1:5200") {
        Ok(r)=>{
            stream = r;
        },
        Err(_)=>{
            return error("failed to start the tcp connection");
        }
    }

    let messages = [
        "hello there",
        "whats up",
        "kali tors"
    ];

    if true {
        for message in messages.iter() {
            send(&mut stream,&message.to_string());
        }
    }

    if false {
        loop {
            for message in messages.iter() {
                send(&mut stream,&message.to_string());
            }
        }
    }

    let mut buffer = [0; 90];
    match stream.read(&mut buffer) {
        Ok(_)=>{
            println!("server message : {:?}",String::from_utf8(buffer.to_vec()));
            //blank(&mut stream);
        },
        Err(_)=>{
            return error("failed to read the server message");
        }
    }



}

fn blank(mut stream:&mut TcpStream){

    let index = 0;

    loop {
        thread::sleep(Duration::from_millis(1000));
        println!("signal");
        stream.write(b"s\r\n");
        let mut buffer = [0; 90];
        match stream.read(&mut buffer) {
            Ok(_)=>{
                println!("server message : {:?}",String::from_utf8(buffer.to_vec()));
                blank(&mut stream);
            },
            Err(_)=>{
                return error("failed to read the server message");
            }
        }
    }

}

fn send(stream:&mut TcpStream,m:&str){

    // let mut id: [u8; 32] = [0; 32];
    // let mut rng = thread_rng();
    // rng.fill(&mut id);

    let encoded = encode(&m.as_bytes());

    // let key:String;
    // match String::from_utf8(id.to_vec()) {
    //     Ok(k)=>{
    //         key = k;
    //     },
    //     Err(e)=>{
    //         println!("parse to string error : {:?}",e);
    //         return;
    //     }
    // }

    let req_id: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .collect();

    let message = format!("SMPL {} {}\r\n",req_id,encoded);

    println!("message : {:?}",message);

    stream.write(message.as_bytes());

}
