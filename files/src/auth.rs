
use actix_web::http::header::HeaderMap;
use crate::common;

pub fn check(headers:&HeaderMap,base_key:String) -> Result<(),String> {

    if headers.contains_key("fdb_app_type") == false {
        return Err(common::error("non_fdb_request"));
    }

    match headers.get("fdb_app_type") {
        Some(app)=>{
            if app == "master" {
                match master(headers,base_key) {
                    Ok(_)=>{return Ok(());},
                    Err(_)=>{
                        return Err(common::error("failed_auth-master"));
                    }
                }
            } else
            if app == "fellow" {
                match fellow(headers,base_key) {
                    Ok(_)=>{return Ok(());},
                    Err(_)=>{
                        return Err(common::error("failed_auth-master"));
                    }
                }
            } else {
                return Err(common::error("invalid_fdb_app_type"));
            }
        },
        None=>{
            return Err(common::error("non_fdb_request"));
        }
    }

}

pub fn master(headers:&HeaderMap,base_key:String) -> Result<(),String> {

    if headers.contains_key("session") == false {
        return Err(common::error("not_found-header-session"));
    }
    if headers.contains_key("timestamp") == false {
        return Err(common::error("not_found-header-timestamp"));
    }
    if headers.contains_key("ruid") == false {
        return Err(common::error("not_found-header-ruid"));
    }
    if headers.contains_key("req_signature") == false {
        return Err(common::error("not_found-header-req_signature"));
    }

    let timestamp:String;
    match headers.get("timestamp") {
        Some(r)=>{timestamp = r.to_str().unwrap().to_string();},
        None=>{return Err(common::error("failed-fetch-header-timestamp"));}
    }
    let ruid:String;
    match headers.get("ruid") {
        Some(r)=>{ruid = r.to_str().unwrap().to_string();},
        None=>{return Err(common::error("failed-fetch-header-ruid"));}
    }
    let req_signature:String;
    match headers.get("req_signature") {
        Some(r)=>{req_signature = r.to_str().unwrap().to_string();},
        None=>{return Err(common::error("failed-fetch-header-req_signature"));}
    }

    let gen_key = common::hash(format!("{}{}{}",timestamp,ruid,base_key));
    if gen_key != req_signature {
        return Err(common::error("invalid-req_signature"));
    }

    Ok(())

}

pub fn fellow(headers:&HeaderMap,base_key:String) -> Result<(),String> {

    Ok(())

}
