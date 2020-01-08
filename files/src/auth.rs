
use actix_web::http::header::HeaderMap;
use std::net::IpAddr;
use crate::common;

pub fn extract_peer(p:core::option::Option<&str>) -> Result<String,String> {

    match p {
        Some(r)=>{
            let peer_as_string = String::from(r);
            if peer_as_string.contains(":") {
                let ips = peer_as_string.split(":").collect::<Vec<&str>>();
                let first_address_as_string = String::from(ips[0]);
                return Ok(first_address_as_string);
            } else {
                return Ok(peer_as_string);
            }
        },
        None=>{
            return Err(common::error("failed-extract_from_option-extract_peer-auth-files"));
        }
    }

}

pub fn check(headers:&HeaderMap,base_key:String,our_node:crate::Node_Template,peer:String
,path:String,composer:crate::Composer_Template) -> Result<(),String> {

    if headers.contains_key("fdb_app_type") == false {
        return Err(common::error("non_fdb_request"));
    }

    match headers.get("fdb_app_type") {
        Some(app)=>{
            if app == "master" {
                match master(headers,composer,peer,path) {
                    Ok(_)=>{return Ok(());},
                    Err(_)=>{
                        return Err(common::error("failed_auth-master"));
                    }
                }
            } else
            if app == "node" {
                match node(headers,our_node,peer,path) {
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

pub fn master(headers:&HeaderMap,composer:crate::Composer_Template,peer:String,path:String) -> Result<(),String> {

    //check if peer is as same as composer
    if peer != composer.ip {
        return Err(common::error("invalid-composer-ip"));
    }

    //check paths for master
    if path != "/check" {
        return Err(common::error("invalid-path_for_master"));
    }

    if headers.contains_key("composer_id") == false {
        return Err(common::error("not_found-header-composer_id"));
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

    let timestamp_as_i64:i64;
    match timestamp.parse::<i64>() {
        Ok(r)=>{
            timestamp_as_i64 = r;
        },
        Err(e)=>{
            println!("e : {:?}",e);
            return Err(common::error("failed-parse_timestamp"));
        }
    }

    let now_as_i64:i64;
    match common::time::now() {
        Ok(t)=>{
            match t.parse::<i64>() {
                Ok(r)=>{
                    now_as_i64 = r;
                },
                Err(_)=>{
                    return Err(common::error("failed-fetch_time"));
                }
            }
        },
        Err(_)=>{
            return Err(common::error("failed-fetch_time"));
        }
    }

    let diff = now_as_i64 - timestamp_as_i64;
    if diff == 0 {
        return Err(common::error("failed-req_expired"));
    }
    if diff < 0 {
        return Err(common::error("failed-invalid_req_timestamp"));
    }
    if diff > 5000 {
        return Err(common::error("failed-req_expired"));
    }

    let gen_key = common::hash(format!("{}{}{}",timestamp,ruid,composer.sig));
    if gen_key != req_signature {
        return Err(common::error("invalid-req_signature-master"));
    }

    Ok(())

}

pub fn node(headers:&HeaderMap,our_node:crate::Node_Template,peer:String,path:String) -> Result<(),String> {

    if path != "/check" && path != "/kill" {
        return Err(common::error("invalid-path_for_master"));
    }

    if headers.contains_key("node_id") == false {
        return Err(common::error("not_found-header-node_id"));
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

    let node_id:String;
    match headers.get("node_id") {
        Some(r)=>{node_id = r.to_str().unwrap().to_string();},
        None=>{return Err(common::error("failed-fetch-header-node_id"));}
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

    let timestamp_as_i64:i64;
    match timestamp.parse::<i64>() {
        Ok(r)=>{
            timestamp_as_i64 = r;
        },
        Err(e)=>{
            println!("e : {:?}",e);
            return Err(common::error("failed-parse_timestamp"));
        }
    }

    let now_as_i64:i64;
    match common::time::now() {
        Ok(t)=>{
            match t.parse::<i64>() {
                Ok(r)=>{
                    now_as_i64 = r;
                },
                Err(_)=>{
                    return Err(common::error("failed-fetch_time"));
                }
            }
        },
        Err(_)=>{
            return Err(common::error("failed-fetch_time"));
        }
    }

    let diff = now_as_i64 - timestamp_as_i64;
    if diff == 0 {
        return Err(common::error("failed-req_expired"));
    }
    if diff < 0 {
        return Err(common::error("failed-invalid_req_timestamp"));
    }
    if diff > 5000 {
        return Err(common::error("failed-req_expired"));
    }

    //println!("our_node_id : {:?}",our_node.id);

    if peer != "127.0.0.1" {
        return Err(common::error("invalid-node_ip_address"));
    }

    if node_id != our_node.id {
        return Err(common::error("invalid-node_id"));
    }

    let gen_key = common::hash(format!("{}{}{}",timestamp,ruid,our_node.sig));
    if gen_key != req_signature {
        return Err(common::error("invalid-req_signature-node"));
    }

    Ok(())

}

pub fn fellow(_headers:&HeaderMap,_base_key:String) -> Result<(),String> {

    Ok(())

}
