use std::net::{TcpListener, TcpStream, SocketAddr, Shutdown};
use std::io::{Write,Read};
use std::time::Duration;
use std::thread;
use base64::{decode,encode};
mod crypt;

pub mod auth;

#[derive(Debug)]
pub struct Request {
    pub r#type:String,
    pub req_id:String,
    pub data:String,
    pub peer:SocketAddr,
    pub raw:String
}

#[derive(Debug)]
pub struct Response {
    pub req_id:String,
    pub action:&'static str,
    pub message:String
}

#[allow(dead_code)]
impl Response {
    #[allow(dead_code)]
    pub fn ok(req:Request) -> Response {
        Response {
            action:"none",
            req_id:req.req_id.clone(),
            message:format!("OK {} none\r\n",req.req_id)
        }
    }
    #[allow(dead_code)]
    pub fn new(req:Request,m:String) -> Result<Response,String> {
        let encoded_message = encode(&m.as_bytes());
        return Ok(
            Response {
                action:"none",
                req_id:req.req_id.clone(),
                message:format!("OK {} {}\r\n",req.req_id,encoded_message)
            }
        );
    }
    #[allow(dead_code)]
    pub fn bad(req:Request) -> Response {
        Response {
            action:"none",
            req_id:req.req_id.clone(),
            message:format!("BAD {} undefined\r\n",req.req_id)
        }
    }
    #[allow(dead_code)]
    pub fn error(req:Request,e:String) -> Response {
        Response {
            action:"none",
            req_id:req.req_id.clone(),
            message:format!("BAD {} {}\r\n",req.req_id,e)
        }
    }
    #[allow(dead_code)]
    pub fn quit(req:Request) -> Response {
        Response {
            action:"quit",
            req_id:req.req_id.clone(),
            message:format!("BYE {}\r\n",req.req_id)
        }
    }
}

pub fn init(address:String,key:String,handler:  fn(Request) -> Result<Response,String>,guard: fn(auth::Token) -> bool ) {

    let key_converted = key.as_bytes().to_vec();

    match TcpListener::bind(address) {
        Ok(listener)=>{
            for listen in listener.incoming() {
                let key_clone = key_converted.clone();
                match listen {
                    Ok(mut stream)=>{
                        thread::spawn(move || {
                            handle_client(&mut stream,&key_clone,handler,guard);
                        });
                    },
                    Err(_)=>{
                        println!("!!! connection failed")
                    }
                }
            }
        },
        Err(_)=>{
            println!("!!! failed to start the server");
        }
    }
}

fn handle_client(stream:&mut TcpStream,key:&Vec<u8>,handler:  fn(Request) -> Result<Response,String>,guard: fn(auth::Token) -> bool  ) {

    let key_as_string:String;
    match String::from_utf8(key.clone()) {
        Ok(parsed)=>{
            key_as_string = parsed.to_string();
        },
        Err(_)=>{
            println!("!!! failed key as vec of u8 to string convertion for auth function");
            return;
        }
    }

    match auth::init(stream,key_as_string) {
        Ok(token)=>{
            if guard(token) == false {
                println!("guard denied access");
                match stream.shutdown(Shutdown::Both) {
                    Ok(_)=>{},
                    Err(_)=>{}
                }
                return;
            } else {
                match stream.write(b"WLCM\r\n") {
                    Ok(_)=>{},
                    Err(_)=>{}
                }
            }
        },
        Err(_)=>{
            match stream.write(b"BYE BAD_AUTH\r\n") {
                Ok(_)=>{},
                Err(_)=>{}
            }
        }
    }

    let peer:SocketAddr;
    let quit_ref = "quit".as_bytes().to_vec();
    match stream.peer_addr() {
        Ok(addr)=>{
            peer = addr;
        },
        Err(_)=>{
            match stream.write(b"BAD - get_peer\r\n") {
                Ok(_)=>{},
                Err(_)=>{}
            }
            match stream.write(b"BYE\r\n") {
                Ok(_)=>{},
                Err(_)=>{}
            }
            match stream.shutdown(Shutdown::Both) {
                Ok(_)=>{},
                Err(_)=>{}
            }
            return;
        }
    }

    match stream.set_read_timeout(Some(Duration::from_millis(1000))) {
        Ok(_)=>{},
        Err(_)=>{}
    }

    let mut overflow = String::new();
    loop {
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

                                    if vec[0].len() == 0 {
                                        line.push_str("\r\n");
                                    }


                                    if true {
                                        //process first message
                                        line.push_str(&vec[0].to_string());
                                        if line.contains("\r\n"){
                                            let line_vec = line.split("\r\n").collect::<Vec<&str>>();
                                            for line_part in line_vec.iter() {
                                                if line_part.len() > 0 {
                                                    let response = &run_request(&peer,line_part.to_string(),key,handler);
                                                    if &quit_ref == response {
                                                        match stream.write(b"BYE\r\n") {
                                                            Ok(_)=>{},
                                                            Err(_)=>{}
                                                        }
                                                        match stream.shutdown(Shutdown::Both) {
                                                            Ok(_)=>{},
                                                            Err(_)=>{}
                                                        }
                                                    } else {
                                                        match stream.write(response) {
                                                            Ok(_)=>{},
                                                            Err(_)=>{}
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            let response = &run_request(&peer,line,key,handler);
                                            if &quit_ref == response {
                                                match stream.write(b"BYE\r\n") {
                                                    Ok(_)=>{},
                                                    Err(_)=>{}
                                                }
                                                match stream.shutdown(Shutdown::Both) {
                                                    Ok(_)=>{},
                                                    Err(_)=>{}
                                                }
                                            } else {
                                                match stream.write(response) {
                                                    Ok(_)=>{},
                                                    Err(_)=>{}
                                                }
                                            }
                                        }
                                    }
                                    if vec[vec_len - 1].len() > 0 {
                                        //overflow = String::from(vec[vec_len - 1]);
                                        overflow.push_str(vec[vec_len - 1]);
                                    }

                                    let mut index = 0;
                                    for incoming in vec {
                                        if index != 0 && index < (vec_len - 1) && incoming.len() > 0 {
                                            let response = &run_request(&peer,incoming.to_string(),key,handler);
                                            if &quit_ref == response {
                                                match stream.write(b"BYE\r\n") {
                                                    Ok(_)=>{},
                                                    Err(_)=>{}
                                                }
                                                match stream.shutdown(Shutdown::Both) {
                                                    Ok(_)=>{},
                                                    Err(_)=>{}
                                                }
                                            } else {
                                                match stream.write(response) {
                                                    Ok(_)=>{},
                                                    Err(_)=>{}
                                                }
                                            }
                                        }

                                        index = index + 1;
                                    }
                                } else if vec_len == &1 {
                                    line.push_str(&vec[0].to_string());
                                    let response = &run_request(&peer,line,key,handler);
                                    if &quit_ref == response {
                                        match stream.write(b"BYE\r\n") {
                                            Ok(_)=>{},
                                            Err(_)=>{}
                                        }
                                        match stream.shutdown(Shutdown::Both) {
                                            Ok(_)=>{},
                                            Err(_)=>{}
                                        }
                                    } else {
                                        match stream.write(response) {
                                            Ok(_)=>{},
                                            Err(_)=>{}
                                        }
                                    }
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
                    match stream.shutdown(Shutdown::Both) {
                        Ok(_)=>{},
                        Err(_)=>{}
                    }
                    break;
                }
            },
            Err(_)=>{
                match stream.write(b"s\r\n") {
                    Ok(_)=>{},
                    Err(_)=>{
                        match stream.shutdown(Shutdown::Both) {
                            Ok(_)=>{},
                            Err(_)=>{}
                        }
                        break;
                    }
                }
            }
        }//match read stream result

        thread::sleep(Duration::from_millis(10));

    }//read loop ends here

}

fn run_request(peer:&SocketAddr,line:String,key:&Vec<u8>,handler:  fn(Request) -> Result<Response,String> ) -> Vec<u8> {

    if line == "s" {
        return "s\r\n".as_bytes().to_vec();
    }

    let peer_clone = peer.clone();
    let key_clone = key.clone();
    let handler_clone = handler.clone();

    let run = thread::spawn(move || {
        match process_request(&peer_clone,line,&key_clone,handler_clone) {
            Ok(response)=>{
                if response.action == "quit" {
                    return "quit".as_bytes().to_vec();
                }
                return response.message.as_bytes().to_vec();
            },
            Err(e)=>{
                return e.to_string().as_bytes().to_vec();
            }
        }
    });

    let result = run.join().unwrap();
    return result;

}

fn process_request(peer:&SocketAddr,line:String,key:&Vec<u8>,handler:  fn(Request) -> Result<Response,String> ) -> Result<Response,String> {
    match parse_request(line.clone(),peer,key) {
        Ok(request)=>{
            match handler(request) {
                Ok(response)=>{
                    return Ok(response);
                },
                Err(e)=>{
                    return Err(format!("BAD {}\r\n",e));
                }
            }
        },
        Err(e)=>{
            let error = format!("BAD request {} {}\r\n",e,line);
            println!("{}",error);
            return Err(error);
        }
    }
}

fn parse_request(line:String,peer:&SocketAddr,key:&Vec<u8>) -> Result<Request,String> {

    let vec = line.split(" ").collect::<Vec<&str>>();
    if vec.len() != 3 {
        return Err("invalid-params".to_string());
    }
    let req_type = vec[0];
    let id = vec[1];
    let data = vec[2];
    if req_type != "SMPL" && req_type != "ECRT" {
        return Err("invalid-request_type".to_string());
    }
    if id.len() != 32 {

        return Err(format!("invalid-id_len => {}",id.len()));
    }
    let processed_data:String;
    if req_type == "ECRT" {
        if data.contains(":") == false {
            return Err("invalid-data_for_ecrt_request-no_seprator".to_string());
        }
        let data_vec = data.split(":").collect::<Vec<&str>>();
        let data_vec_len = &data_vec.len();
        if data_vec_len != &2 {
            return Err("invalid-params".to_string());
        }
        if data_vec[0].len() != 14 {
            return Err("invalid-data_for_ecrt_request-invalid_nonce".to_string());
        }
        match decode(data_vec[1]) {
            Ok(decoded)=>{
                match crypt::decrypt(decoded, key, data_vec[0].as_bytes().to_vec()) {
                    Ok(r)=>{
                        processed_data = r;
                    },
                    Err(_)=>{
                        return Err("failed-decypt-decoded_base64_string".to_string());
                    }
                }
            },
            Err(_)=>{
                return Err("invalid-base64_string".to_string());
            }
        }
    } else {
        match decode(data) {
            Ok(decoded)=>{
                match String::from_utf8(decoded) {
                    Ok(decoded_message)=>{
                        processed_data = decoded_message;
                    },
                    Err(_) => {
                        return Err("invalid-parse_to_string-base64_decoded_vec".to_string());
                    }
                }
            },
            Err(_)=>{
                return Err("invalid-base64_string".to_string());
            }
        }
    }

    //println!("decoded : {:?}",&processed_data);

    let build = Request {
        req_id:id.to_string(),
        data:processed_data,
        r#type:req_type.to_string(),
        peer:peer.clone(),
        raw:line
    };

    return Ok(build);

}
