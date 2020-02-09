use std::time::{Instant, SystemTime, UNIX_EPOCH, Duration};
use std::net::{TcpStream,SocketAddr};
use sha2::Sha256;
use std::thread;
use hmac::{Hmac, Mac};
use std::io::Read;
use base64::decode;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug)]
pub struct Token {
    pub id:String,
    pub time:String,
    pub peer:SocketAddr
}

pub fn init(stream:&mut TcpStream,key:String) -> Result<Token,String> {

    let peer:SocketAddr;
    match stream.peer_addr() {
        Ok(addr)=>{
            peer = addr;
        },
        Err(_)=>{
            return Err("failed-fetch_peer".to_string());
        }
    }

    match stream.set_read_timeout(Some(Duration::from_millis(500))) {
        Ok(_)=>{},
        Err(_)=>{}
    }

    let mut line = String::new();
    let initial = Instant::now();

    loop {

        if initial.elapsed().as_secs() < 10 {
            let mut buffer = [0; 32];
            let buffer_ref = [0; 32];
            match stream.read(&mut buffer){
                Ok(_)=>{
                    if buffer != buffer_ref {
                        let mut clean = Vec::new();
                        for byte in buffer.iter() {
                            if byte != &0 {
                                clean.push(byte.clone());
                            }
                        }
                        match String::from_utf8(clean) {
                            Ok(m)=>{
                                if m.contains("\r\n") {
                                    let vec = m.split("\r\n").collect::<Vec<&str>>();
                                    line.push_str(&vec[0].to_string());
                                    break;
                                } else {
                                    line.push_str(&m);
                                }
                            },
                            Err(_)=>{}
                        }
                    }
                },
                Err(_)=>{}
            }
        } else {
            break;
        }

        thread::sleep(Duration::from_millis(10));

    }

    match parse_auth_request(line, key, peer) {
        Ok(token)=>{
            return Ok(token);
        },
        Err(e)=>{
            println!("auth error : {:?}",e);
            return Err("failed to verify token request".to_string());
        }
    }

}

fn parse_auth_request(line:String,key:String,peer:SocketAddr) -> Result<Token,String> {

    let vec = line.split(" ").collect::<Vec<&str>>();
    let vec_len = vec.len();

    if vec_len != 5 {
        return Err(String::from("invalid_request_length-auth_request"));
    }

    let mut token = Token {
        id:String::new(),
        time:String::new(),
        peer:peer
    };

    if vec[0] != "AUTH" {
        return Err(String::from("invalid_request_header-auth_request"));
    }

    match vec[1].parse::<i64>() {
        Ok(since)=>{
            match get_time() {
                Ok(now)=>{
                    let diff = now - since;
                    if diff > 10000 {
                        return Err("request_expired".to_string());
                    }
                },
                Err(_)=>{
                    return Err("failed-get_current_time".to_string());
                }
            }
        },
        Err(_)=>{
            return Err(String::from("invalid_request_time-auth_request"));
        }
    }

    let combined = format!("{}{}",vec[1],vec[2]);
    match decode(vec[3]) {
        Ok(signature_as_bytes)=>{
            match HmacSha256::new_varkey(key.as_bytes()) {
                Ok(mut signer)=>{
                    signer.input(combined.as_bytes());
                    match signer.verify(&signature_as_bytes) {
                        Ok(_)=>{},
                        Err(_)=>{
                            return Err(String::from("failed-verify_hmac-auth_request"));
                        }
                    }
                },
                Err(_)=>{
                    return Err("failed to init hmac signer".to_string());
                }
            }
        },
        Err(_)=>{
            return Err(String::from("failed-decode_signature_to_bytes-auth_request"));
        }
    }

    token.time = vec[1].to_string();
    token.id = vec[4].to_string();

    return Ok(token);

}

fn get_time() -> Result<i64,String> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(v)=>{
            let as_string = v.as_millis().to_string();
            match as_string.parse::<i64>() {
                Ok(time)=>{
                    return Ok(time);
                },
                Err(_)=>{
                    return Err("failed parse time to i64 form string".to_string());
                }
            }
        },
        Err(e)=>{
            let error = format!("error in fetch time since unix epoch {:?}",e);
            return Err(error);
        }
    }
}
