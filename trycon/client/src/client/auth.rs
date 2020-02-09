use sha2::Sha256;
use hmac::{Hmac, Mac};
use std::net::TcpStream;
use std::io::{Write,Read};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use base64;
use std::time::{Instant, SystemTime, UNIX_EPOCH, Duration};

type HmacSha256 = Hmac<Sha256>;

pub fn init(stream:&mut TcpStream,connection_id:&String,key:String) -> Result<(),String> {

    let anchor: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .collect();

    let time:String;
    match get_time() {
        Ok(get)=>{
            time = get;
        },
        Err(_)=>{
            return Err("failed to get time".to_string());
        }
    }

    let combined = format!("{}{}",time,anchor);
    let signature:String;
    match HmacSha256::new_varkey(key.as_bytes()) {
        Ok(mut signer)=>{
            signer.input(combined.as_bytes());
            let result = signer.result();
            let signature_vec = result.code();
            signature = base64::encode(&signature_vec);
        },
        Err(_)=>{
            return Err("failed to init hmac signer".to_string());
        }
    }

    let parse = format!("AUTH {} {} {} {}\r\n",time,anchor,signature,&connection_id);
    match stream.write(&parse.as_bytes()) {
        Ok(_)=>{},
        Err(_)=>{}
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
                        match String::from_utf8(buffer.to_vec()) {
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

    }

    if line.contains("WLCM") == false {
        return Err("didi not receive welcome message form the server".to_string());
    }

    return Ok(());

}

fn get_time() -> Result<String,String> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(v)=>{
            let parse = v.as_millis().to_string();
            return Ok(parse);
        },
        Err(e)=>{
            let error = format!("error in fetch time since unix epoch {:?}",e);
            return Err(error);
        }
    }
}
