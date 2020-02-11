use crate::client::{Response,RESPONSES,REQUESTS,Request,crypt,common};
use base64::decode;

pub fn get_requests(id:&String) -> Result<Vec<Request>,String>{
    match REQUESTS.lock() {
        Ok(mut pool)=>{
            if pool.contains_key(id) == false {
                return Err(String::from("no_requests"));
            }
            match pool.get_mut(id) {
                Some(request_pool)=>{
                    let hold_this = request_pool.clone();
                    request_pool.clear();
                    return Ok(hold_this);
                },
                None=>{
                    return Err(String::from("failed-get_mut_id_array_from_REQUESTS"));
                }
            }
        },
        Err(_)=>{
            return Err(String::from("failed-lock_REQUESTS"));
        }
    }
}

pub fn poll_response(id:&String) -> Result<Response,String> {
    match RESPONSES.lock() {
        Ok(mut resp_pool)=>{
            if resp_pool.contains_key(id) {
                match resp_pool.get(id) {
                    Some(response)=>{
                        let response_holder = response.clone();
                        match resp_pool.remove(id) {
                            Some(_)=>{},
                            None=>{
                                return Err(common::error("failed-remove_response_from_pool"));
                            }
                        }
                        return Ok(response_holder);
                    },
                    None=>{
                        return Err(common::error("failed-extract_response"));
                    }
                }
            } else {
                return Err(String::from("failed-response-not_found"));
            }
        },
        Err(_)=>{
            return Err(common::error("failed-lock_responses_pool"));
        }
    }
}

pub fn parse(line:String,key:&String) -> Result<Response,String> {

    let vec = line.split(" ").collect::<Vec<&str>>();
    let vec_len = vec.len();
    if vec_len > 3 ||vec_len < 2 {
        return Err("invalid_message-length".to_string());
    }

    let one = vec[0];
    let parts = vec.len();

    if one != "OK" && one != "BAD" && one != "BYE" {
        return Err("invalid-response_type".to_string());
    }
    if one == "BYE" {
        return Err("connection-closed".to_string());
    }

    if one == "BAD" {
        if parts == 1 {
            return Err("undefined-error".to_string());
        } else if parts == 2 {
            return Err("undefined-request_id".to_string());
        }
    }

    if one == "BAD" {
        if parts == 3 {
            return Ok(Response {
                result:false,
                req_id:vec[1].to_string(),
                message:String::new(),
                error:vec[2].to_string(),
                request:Request::new()
            });
        } else {
            return Err("undefined-error-for_bad-parts".to_string());
        }
    } else if one == "OK" {
        if parts == 2 {
            return Ok(Response {
                result:false,
                req_id:vec[1].to_string(),
                message:String::from("undefined"),
                error:String::new(),
                request:Request::new()
            });
        } else if parts == 3 {
            match decode_body(vec[2].to_string(),key){
                Ok(message)=>{
                    return Ok(Response {
                        result:false,
                        req_id:vec[1].to_string(),
                        message:message,
                        error:String::new(),
                        request:Request::new()
                    });
                },
                Err(e)=>{
                    let error = format!("failed-decode-message_body error : {}",e);
                    return Err(error);
                }
            }
        } else {
            return Err("undefined-error-for_ok-parts".to_string());
        }
    } else {
        return Err("undefined-error-for_ok".to_string());
    }

}

fn decode_body(line:String,key:&String) -> Result<String,String> {

    if line.contains(":") {
        match decrypt_message(&line, key) {
            Ok(r)=>{
                return Ok(r.to_string());
            },
            Err(e)=>{
                let error = format!("failed-decrypt_message error : {}",e);
                return Err(error);
            }
        }
    }

    match decode(&line) {
        Ok(r)=>{
            match String::from_utf8(r) {
                Ok(result)=>{
                    return Ok(result);
                },
                Err(_)=>{
                    return Err(String::from("failed-parse-decoded_message"));
                }
            }
        },
        Err(_)=>{
            return Err(String::from("failed-decode_message"));
        }
    }

}

fn decrypt_message(line:&String,key:&String) -> Result<String,String> {

    let vec = line.split(":").collect::<Vec<&str>>();
    let vec_len = vec.len();
    if vec_len != 2 {
        return Err(String::from("invalid-encrypted_message_format"));
    }

    let nonce:Vec<u8>;
    match decode(vec[0]) {
        Ok(nonce_as_array)=>{
            nonce = nonce_as_array.to_vec();
        },
        Err(_)=>{
            return Err(String::from("failed-decode_nonce"));
        }
    }

    let cipher:Vec<u8>;
    match decode(vec[1]) {
        Ok(cipher_as_array)=>{
            cipher = cipher_as_array.to_vec();
        },
        Err(_)=>{
            return Err(String::from("failed-decode_cipher"));
        }
    }

    let key_clone = key.clone();
    let key_as_vector = key_clone.as_bytes().to_vec();
    match crypt::decrypt(cipher,&key_as_vector,nonce) {
        Ok(message)=>{
            return Ok(message);
        },
        Err(e)=>{
            let error = format!("failed-decrypt_decoded_cipher : {}",e);
            return Err(error);
        }
    }

}
