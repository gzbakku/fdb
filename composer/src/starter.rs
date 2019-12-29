use std::process::{Command, Stdio};
use crate::common;
use crate::io;

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
    let mut composer_address:String = String::from("127.0.0.1:8080");
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

        if &fmt.r#type == &"composer".to_string() {
            composer_address = format!("127.0.0.1:{}",fmt.port);
        }

        collect_actors.push(fmt);

    }

    if built == false {
        return Err(common::error("failed-parse_actors"));
    }

    let session = common::new_session();

    let app_dir = io::app_dir();
    let mut started = true;
    for a in &collect_actors {
        match start_actor(a,&secure,&base_dir,&app_dir,&session,&composer_address) {
            Ok(_)=>{},
            Err(_) => {
                println!("{:?}",a);
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

fn start_actor(actor:&Actor,secure:&String,base_dir:&String,app_dir:&String,session:&common::SESSION,composer:&String) -> Result<(),String> {

    //println!("actor : {:?}",actor);

    if actor.r#type == "files".to_string() {

        //println!("app_dir : {:?}",&app_dir);

        let actor_path = format!("{}{} --secure={} --signature={} --id={} --session_id={} --session_signature={} --base_dir={} --port={} --composer={}",&app_dir,&actor.r#type,&secure,&actor.sig,&actor.id,&session.id,&session.sig,&base_dir,&&actor.port,&composer);

        //println!("actor_path : {:?}",&actor_path);

        println!("");println!("");println!("");

        match
        Command::new("cmd")
        .arg("/k")
        .arg(&actor_path)
        .spawn()
        {
            Ok(mut process)=>{
                println!("process : {:?}",process);
            },
            Err(e) => {
                println!("failed to spawn process e : {:?}",e);
                return Err(common::error("failed-spawn_actor"));
            }
        }

    }

    Ok(())

}

/*
    --secure=Om2lPq84vgIhsPEhWsh3LdRmNmI2MXpQ --signature=XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb --id=aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5
    --session_id=XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb --session_signature=aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5 --base_dir=d://workstation/expo/rust/fdb/composer/instance --port=8088 --composer=127.0.0.1

    D:\\workstation\\expo\\rust\\fdb\\composer\\target\\debug\\files --secure=FhE1qxPiUSE5QMCAvDCIFAQxjVAvmkVR --signature=XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb --id=aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5 --session_id=THR4FyC7LuMqdQT2jh5aOikHkp0wCNq3 --session_signature=e78ZTtfoQYWQLqgmrNDUf0OWV4dCOqJc --base_dir=D:\\workstation\\expo\\rust\\fdb\\composer/instance/ --port=8088 --composer=127.0.0.1:5200

*/
