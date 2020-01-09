use std::process::Command;
use std::collections::HashMap;
use crate::common;
use crate::io;
use json;

#[derive(Debug)]
pub struct Actor {
    pub r#type:String,
    pub id:String,
    pub sig:String,
    pub port:i32,
}

pub fn init(e:&io::Extracted) -> Result<Vec<Actor>,String> {

    common::log("initiating starter");

    let secure = &e.password;
    let config = &e.config;
    let base_dir = &e.base_dir;

    common::log("base directory verified");

    let mut collect_actors = Vec::new();
    //let mut composer_address:String = String::from("127.0.0.1:8080");
    let mut built = true;
    for object in config["actors"].members() {

        let mut fmt = Actor {
            id:String::new(),
            sig:String::new(),
            port:0,
            r#type:String::new()
        };

        match object["id"].as_str() {
            Some(r) => {fmt.id = r.to_string();},
            None=>{built = false;common::error("failed-parse_actor-id");break;}
        };
        match object["signature"].as_str() {
            Some(r) => {fmt.sig = r.to_string();},
            None=>{built = false;common::error("failed-parse_actor-signature");break;}
        };
        match object["port"].as_i32() {
            Some(r) => {fmt.port = r;},
            None=>{built = false;common::error("failed-parse_actor-port");break;}
        };
        match object["type"].as_str() {
            Some(r) => {fmt.r#type = r.to_string();},
            None=>{built = false;common::error("failed-parse_actor-type");break;}
        };

        // if &fmt.r#type == &"composer".to_string() {
        //     composer_address = format!("127.0.0.1:{}",fmt.port);
        // }

        collect_actors.push(fmt);

    }

    if built == false {
        return Err(common::error("failed-parse_actors"));
    }

    let session = &e.session;
    let composer = &e.composer;
    let node = &e.node;

    let app_dir = io::app_dir();
    let mut started = true;
    for a in &collect_actors {
        match start_actor(a,&secure,&base_dir,&app_dir,&session,&composer,&node) {
            Ok(_)=>{},
            Err(_) => {
                common::error("actor failed to initiate");
                started = false;
                break;
            }
        }
    }

    if started == false {
        return Err(common::error("failed-start_actors"));
    }

    common::log("actors initiated");

    Ok(collect_actors)

}

fn start_actor(actor:&Actor,secure:&String,base_dir:&String,app_dir:&String,session:&io::Session,composer:&io::Composer,node:&io::Node) -> Result<(),String> {

    if actor.r#type == "files".to_string() {

        let actor_path = format!(
            "{}{} --secure={} --actor_signature={} --actor_id={} --session_id={} --session_signature={} --node_signature={} --node_id={} --node_port={} --composer_signature={} --composer_id={} --composer_ip={} --composer_port={} --base_dir={} --port={}",
                &app_dir,&actor.r#type,&secure,
                &actor.sig,&actor.id,
                &session.id,&session.sig,
                &node.sig,&node.id,&node.port,
                &composer.sig,&composer.id,&composer.ip,&composer.port,
                &base_dir,&&actor.port
        );

        //println!("{}",&actor_path);

        match
        Command::new("cmd")
        .arg("/k")
        .arg(&actor_path)
        .spawn()
        {
            Ok(_)=>{
                match check_actor(&actor,&node) {
                    Ok(_)=>{
                        return  Ok(());
                    },
                    Err(_)=>{
                        return Err(common::error("failed-check_actor"));
                    }
                }
            },
            Err(e) => {
                println!("failed to spawn process e : {:?}",e);
                return Err(common::error("failed-spawn_actor"));
            }
        }

    }

    Ok(())

}

fn check_actor(actor:&Actor,node:&io::Node) -> Result<(),String> {

    let ruid = common::uid();
    let timestamp:String;
    match common::time::now() {
        Ok(r)=>{timestamp = r;},
        Err(_)=>{
            return Err(common::error("failed-fetch_timestamp"));
        }
    }

    let url = format!("http://127.0.0.1:{}/check",actor.port);

    let req_signature = common::hash(format!("{}{}{}",&timestamp,&ruid,&node.sig));

    let mut headers = HashMap::new();
    headers.insert("fdb_app_type","node".to_string());
    headers.insert("timestamp",timestamp);
    headers.insert("ruid",ruid);
    headers.insert("node_id",node.id.clone());
    headers.insert("req_signature",req_signature);

    let obj = json::object!{
        "req_type" => "check"
    };

    match common::request::post::object(url,obj,headers) {
        Ok(r)=>{
            if r.success == false {
                return Err(common::error("failed-check_actor_request"));
            } else {
                common::log_string(format!("actor checked : {:?}",actor.r#type));
                return Ok(());
            }
        },
        Err(_)=>{
            return Err(common::error("failed-send_check_actor_request"));
        }
    }

}

/*
    --secure=Om2lPq84vgIhsPEhWsh3LdRmNmI2MXpQ --signature=XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb --id=aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5
    --session_id=XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb --session_signature=aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5 --base_dir=d://workstation/expo/rust/fdb/composer/instance --port=8088 --composer=127.0.0.1

    D:\\workstation\\expo\\rust\\fdb\\composer\\target\\debug\\files --secure=FhE1qxPiUSE5QMCAvDCIFAQxjVAvmkVR --signature=XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb --id=aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5 --session_id=THR4FyC7LuMqdQT2jh5aOikHkp0wCNq3 --session_signature=e78ZTtfoQYWQLqgmrNDUf0OWV4dCOqJc --base_dir=D:\\workstation\\expo\\rust\\fdb\\composer/instance/ --port=8088 --composer=127.0.0.1:5200

*/
