use lazy_static::lazy_static;
use std::sync::Mutex;
use std::net::{TcpStream,Shutdown};
use std::io::{Read,Write};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::time::Duration;
use std::thread;
use std::collections::HashMap;
use base64::{encode};
use json::{JsonValue,parse};
use mio::{/*Events,*/ Token, Waker, Poll};

mod crypt;
mod comm;
pub mod common;
mod auth;
pub mod channel;

//#[allow(dead_code)]

#[derive(Clone, Debug)]
pub struct Request {
    pub req_id:String,
    pub message:String,
    pub parsed:String
}

impl Request {
    pub fn new() -> Request{
        Request {
            req_id:String::new(),
            message:String::new(),
            parsed:String::new()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Response {
    pub result:bool,
    pub req_id:String,
    pub message:String,
    pub error:String,
    pub request:Request
}

#[allow(dead_code)]
impl Response {
    #[allow(dead_code)]
    pub fn new(req:Request,message:String) -> Response {
        Response {
            result:true,
            req_id:req.req_id.clone(),
            message:message,
            error:String::new(),
            request:req
        }
    }
    #[allow(dead_code)]
    pub fn error(req:Request,e:String) -> Response {
        Response {
            result:false,
            req_id:req.req_id.clone(),
            message:String::new(),
            error:e,
            request:req
        }
    }
    #[allow(dead_code)]
    pub fn parse_to_json(self) -> Result<JsonValue,&'static str>{
        match parse(&self.message){
            Ok(m)=>{
                return Ok(m);
            },
            Err(_)=>{
                return Err("failed-parse_str_to_json");
            }
        }
    }
}

lazy_static! {
    #[derive(Debug)]
    static ref KEYS : Mutex<HashMap<String,String>> = Mutex::new(HashMap::new());
    static ref CONNECTIONS : Mutex<HashMap<String,bool>> = Mutex::new(HashMap::new());
    static ref REQUESTS : Mutex<HashMap<String,Vec<Request>>> = Mutex::new(HashMap::new());
    static ref RESPONSES : Mutex<HashMap<String,Response>> = Mutex::new(HashMap::new());
    static ref TIMEOUT : Mutex<Vec<u8>> = Mutex::new(Vec::new());
    static ref TRIGGERS : Mutex<HashMap<String,Waker>> = Mutex::new(HashMap::new());
}

#[allow(dead_code)]
pub fn send_message_sync(id_base:&String,message:String,secure:bool) -> Result<Response,&'static str> {

    let id = id_base.clone();
    let req_id: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .collect();

    let mut func = "SMPL";
    let encoded:String;
    if secure {
        match KEYS.lock() {
            Ok(keys)=>{
                match keys.get(&id) {
                    Some(key)=>{
                        encoded = crypt::encode_encrypt_message(message.clone(),key.to_string());
                    },
                    None=>{
                        return Err("failed-extract_key-from_KEYS_pool");
                    }
                }
            },
            Err(_)=>{
                return Err("failed-lock-connection_keys");
            }
        }
        func = "ECRT";
    } else {
        encoded = encode(&message);
    }

    let parsed = format!("{} {} {}\r\n",func,&req_id,encoded);
    let request = Request {
        req_id:req_id.clone(),
        message:message.clone(),
        parsed:parsed
    };

    match REQUESTS.lock() {
        Ok(mut request_pool)=>{
            if request_pool.contains_key(&id) == false {
                request_pool.insert(id.clone(),Vec::new());
            }
            match request_pool.get_mut(&id) {
                Some(connection_pool) => {
                    connection_pool.push(request.clone());
                },
                None=>{
                    return Err("failed-get-connection_id_pool");
                }
            }
        },
        Err(_)=>{
            return Err("failed-lock-Requests");
        }
    }

    //*********************************************
    //make token

    let poll:Poll;
    match Poll::new(){
        Ok(o)=>{
            poll = o;
        },
        Err(_)=>{
            return Err("failed-init-poll");
        }
    }

    // let events = Events::with_capacity(2);
    const WAKE_TOKEN: Token = Token(10);

    match Waker::new(poll.registry(), WAKE_TOKEN){
        Ok(waker)=>{
            match TRIGGERS.lock(){
                Ok(mut lock)=>{
                    match lock.insert(req_id.clone(),waker){
                        Some(_)=>{},
                        None=>{}
                    }
                },
                Err(_)=>{
                    return Err("failed-lock-triggers");
                }
            }
        },
        Err(_)=>{
            return Err("failed-make-waker");
        }
    }

    //poll the response

    let mut index:u16 = 0;
    loop {
        match comm::poll_response(&req_id) {
            Ok(mut response)=>{
                response.request = request;
                return Ok(response);
            },
            Err(_)=>{}
        }
        if index >= 5000 {
            break;
        } else {
            index += 10;
        }
        thread::sleep(Duration::from_millis(30));
    }

    return Err("failed-poll-response");

}

#[allow(dead_code)]
pub async fn send_message_async(id_base:&String,message:String,secure:bool) -> Result<Response,&'static str> {

    let id = id_base.clone();
    let req_id: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .collect();

    let mut func = "SMPL";
    let encoded:String;
    if secure {
        match KEYS.lock() {
            Ok(keys)=>{
                match keys.get(&id) {
                    Some(key)=>{
                        encoded = crypt::encode_encrypt_message(message.clone(),key.to_string());
                    },
                    None=>{
                        return Err("failed-extract_key-from_KEYS_pool");
                    }
                }
            },
            Err(_)=>{
                return Err("failed-lock-connection_keys");
            }
        }
        func = "ECRT";
    } else {
        encoded = encode(&message);
    }

    let parsed = format!("{} {} {}\r\n",func,&req_id,encoded);
    let request = Request {
        req_id:req_id.clone(),
        message:message.clone(),
        parsed:parsed
    };

    match REQUESTS.lock() {
        Ok(mut request_pool)=>{
            if request_pool.contains_key(&id) == false {
                request_pool.insert(id.clone(),Vec::new());
            }
            match request_pool.get_mut(&id) {
                Some(connection_pool) => {
                    connection_pool.push(request.clone());
                },
                None=>{
                    return Err("failed-get-connection_id_pool");
                }
            }
        },
        Err(_)=>{
            return Err("failed-lock-Requests");
        }
    }

    //*********************************************
    //make token

    let poll:Poll;
    match Poll::new(){
        Ok(o)=>{
            poll = o;
        },
        Err(_)=>{
            return Err("failed-init-poll");
        }
    }

    // let events = Events::with_capacity(2);
    const WAKE_TOKEN: Token = Token(10);

    match Waker::new(poll.registry(), WAKE_TOKEN){
        Ok(waker)=>{
            match TRIGGERS.lock(){
                Ok(mut lock)=>{
                    match lock.insert(req_id.clone(),waker){
                        Some(_)=>{},
                        None=>{}
                    }
                },
                Err(_)=>{
                    return Err("failed-lock-triggers");
                }
            }
        },
        Err(_)=>{
            return Err("failed-make-waker");
        }
    }

    //poll the response

    let mut index:u16 = 0;
    loop {
        match comm::poll_response(&req_id) {
            Ok(mut response)=>{
                response.request = request;
                return Ok(response);
            },
            Err(_)=>{}
        }
        if index >= 5000 {
            break;
        } else {
            index += 10;
        }
        thread::sleep(Duration::from_millis(30));
    }

    return Err("failed-poll-response");

}

#[allow(dead_code)]
pub fn send_message(id_base:&String,message:String,secure:bool) -> Result<Response,String> {

    let id = id_base.clone();
    let handler = thread::spawn(move || {

        let req_id: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .collect();

        let mut func = "SMPL";
        let encoded:String;
        if secure {
            match KEYS.lock() {
                Ok(keys)=>{
                    match keys.get(&id) {
                        Some(key)=>{
                            encoded = crypt::encode_encrypt_message(message.clone(),key.to_string());
                        },
                        None=>{
                            return Err("failed-extract_key-from_KEYS_pool");
                        }
                    }
                },
                Err(_)=>{
                    return Err("failed-lock-connection_keys");
                }
            }
            func = "ECRT";
        } else {
            encoded = encode(&message);
        }

        let parsed = format!("{} {} {}\r\n",func,&req_id,encoded);
        let request = Request {
            req_id:req_id.clone(),
            message:message.clone(),
            parsed:parsed
        };

        match REQUESTS.lock() {
            Ok(mut request_pool)=>{
                if request_pool.contains_key(&id) == false {
                    request_pool.insert(id.clone(),Vec::new());
                }
                match request_pool.get_mut(&id) {
                    Some(connection_pool) => {
                        connection_pool.push(request.clone());
                    },
                    None=>{
                        return Err("failed-get-connection_id_pool");
                    }
                }
            },
            Err(_)=>{
                return Err("failed-lock-Requests");
            }
        }

        //*********************************************
        //make token

        let poll:Poll;
        match Poll::new(){
            Ok(o)=>{
                poll = o;
            },
            Err(_)=>{
                return Err("failed-init-poll");
            }
        }

        // let events = Events::with_capacity(2);
        const WAKE_TOKEN: Token = Token(10);

        match Waker::new(poll.registry(), WAKE_TOKEN){
            Ok(waker)=>{
                match TRIGGERS.lock(){
                    Ok(mut lock)=>{
                        match lock.insert(req_id.clone(),waker){
                            Some(_)=>{},
                            None=>{}
                        }
                    },
                    Err(_)=>{
                        return Err("failed-lock-triggers");
                    }
                }
            },
            Err(_)=>{
                return Err("failed-make-waker");
            }
        }

        //poll the response

        let mut index:u16 = 0;
        loop {
            match comm::poll_response(&req_id) {
                Ok(mut response)=>{
                    response.request = request;
                    return Ok(response);
                },
                Err(_)=>{}
            }
            if index >= 5000 {
                break;
            } else {
                index += 10;
            }
            thread::sleep(Duration::from_millis(30));
        }

        return Err("failed-poll-response");

    });

    match handler.join() {
        Ok(result)=>{
            match result {
                Ok(response)=>{
                    return Ok(response);
                },
                Err(_)=>{
                    return Err(String::from("failed to rejoin the thread"));
                }
            }
        },
        Err(_)=>{
            return Err(String::from("failed to rejoin the thread"));
        }
    }

}

pub fn get_random_connection_id() -> String {
    let connection_id: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .collect();
    return connection_id;
}

pub fn get_test_message(len:usize) -> String {
    let message: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .collect();
    return message;
}

pub fn start_connection(id:&String,address:String,key:String) -> Result<(),String> {

    match KEYS.lock() {
        Ok(mut pool)=>{
            if pool.contains_key(id) == false {
                match pool.insert(id.clone(),key.clone()) {
                    Some(_)=>{},
                    None=>{}
                }
            }
        },
        Err(_)=>{
            return Err(common::error("failed-lock_KEYS_pool"));
        }
    }

    match TcpStream::connect(address) {
        Ok(mut r)=>{
            let id_holder = id.to_string();
            thread::spawn(move || {
                match do_auth(&mut r,id_holder,key) {
                    Ok(_)=>{},
                    Err(_)=>{}
                }
            });
            return Ok(());
        },
        Err(_)=>{
            return Err(common::error("failed-TcpStream_connect"));
        }
    }
}

fn do_auth(stream:&mut TcpStream,connection_id:String,key:String) -> Result<(),String> {

    match auth::init(stream,&connection_id,key.clone()) {
        Ok(_)=>{
            handle_connection(stream, connection_id, key);
            return Ok(());
        },
        Err(e)=>{
            match stream.shutdown(Shutdown::Both) {
                Ok(_)=>{},
                Err(_)=>{}
            }
            println!("!!! auth error : {:?}",e);
            return Err("auth failed".to_string());
        }
    }

}

fn handle_connection(stream:&mut TcpStream,connection_id:String,key:String){

    match stream.set_read_timeout(Some(Duration::from_millis(10))) {
        Ok(_)=>{},
        Err(_)=>{}
    }

    let mut overflow = String::new();
    loop {

        match comm::get_requests(&connection_id){
            Ok(requests)=>{
                for request in requests {
                    match stream.write(request.parsed.as_bytes()) {
                        Ok(_)=>{},
                        Err(_)=>{}
                    }
                }
            },
            Err(_)=>{}
        }

        let mut line = String::new();
        if overflow.len() > 0 {
            line.push_str(&overflow);
            overflow = String::new();
        }
        let buffer_ref = [0; 32];
        let mut buffer = [0; 32];
        match stream.read(&mut buffer) {
            Ok(_)=>{
                if buffer_ref != buffer {
                    let mut collect_cleaned_buffer = Vec::new();
                    for byte in buffer.iter() {
                        if byte != &0 {
                            collect_cleaned_buffer.push(byte.clone());
                        }
                    }
                    match String::from_utf8(collect_cleaned_buffer) {
                        Ok(m)=>{
                            //complete message found
                            if m.contains("\r\n"){
                                let vec = m.split("\r\n").collect::<Vec<&str>>();
                                let vec_len = &vec.len();
                                if vec_len > &1 {
                                    //mulitiple messages in one read array
                                    if vec[0].len() == 0 {
                                        line.push_str("\r\n");
                                    }
                                    if true {
                                        //process first message
                                        if vec[0].len() != 0 {
                                            line.push_str(&vec[0].to_string());
                                        }
                                        if line.contains("\r\n"){
                                            let line_vec = line.split("\r\n").collect::<Vec<&str>>();
                                            for line_part in line_vec.iter() {
                                                if line_part.len() > 0 {
                                                    process_response(line_part.to_string(),&key);
                                                }
                                            }
                                        } else {
                                            process_response(line,&key);
                                        }
                                    }
                                    if vec[vec_len - 1].len() > 0 {
                                        overflow.push_str(vec[vec_len - 1]);
                                    }
                                    let mut index = 0;
                                    for incoming in vec {
                                        if index != 0 && index < (vec_len - 1) && incoming.len() > 0 {
                                            process_response(incoming.to_string(),&key);
                                        }
                                        index = index + 1;
                                    }
                                } else if vec_len == &1 {
                                    line.push_str(&vec[0].to_string());
                                    process_response(line,&key);
                                }
                            } else {
                                //no request seprator
                                overflow.push_str(&line);
                                overflow.push_str(&m);
                            }
                        },
                        Err(_)=>{
                            match stream.write(b"BAD failed-parse_string_from_buffer\r\n") {
                                Ok(_)=>{},
                                Err(_)=>{}
                            }
                        }
                    }//buffer to stirng converstion

                } else { //make buffer to ref buffer
                    // match stream.shutdown(Shutdown::Both) {
                    //     Ok(_)=>{},
                    //     Err(_)=>{}
                    // }
                    // break;
                }
            },
            Err(_)=>{}
        }//match read stream result

        match stream.write(b"s\r\n") {
            Ok(_)=>{},
            Err(_)=>{}
        }

        thread::sleep(Duration::from_millis(10));

    }//read loop ends here

}

fn process_response(line:String,key:&String){

    //println!("line: {:?}",&line);

    if line == "s" {
        return;
    }

    match comm::parse(line.clone(),key) {
        Ok(response)=>{
            match RESPONSES.lock() {
                Ok(mut pool)=>{
                    match TRIGGERS.lock(){
                        Ok(mut lock)=>{
                            match lock.get_mut(&response.req_id){
                                Some(waker)=>{
                                    match waker.wake(){
                                        Ok(_)=>{
                                            match lock.remove(&response.req_id){
                                                Some(_)=>{},
                                                None=>{}
                                            }
                                        },
                                        Err(_)=>{
                                            common::error("failed-lock-triggers-parse_response-client-postoffice");
                                        }
                                    }
                                },
                                None=>{
                                    common::error("failed-lock-triggers-parse_response-client-postoffice");
                                }
                            }
                        },
                        Err(_)=>{
                            common::error("failed-lock-triggers-parse_response-client-postoffice");
                        }
                    }
                    pool.insert(response.req_id.clone(),response);
                },
                Err(_)=>{
                    common::error("failed-lock-responses-parse_response-client-postoffice");
                }
            }
        },
        Err(e)=>{
            println!("!!! line : {:?}",line);
            println!("!!! error : {:?}",e);
            common::error("failed-process_response--parse_response-client-postoffice");
        }
    }
}
