use crate::common;
use crate::io;

#[derive(Debug)]
pub struct Actor {
    pub r#type:String,
    pub id:String,
    pub signature:String,
    pub port:i32,
}

pub fn init(e:&io::Extracted) -> Result<Vec<Actor>,String> {

    common::log("initiating starter");

    let secure = &e.password;
    let config = &e.config;
    let base_dir = &e.base_dir;

    common::log("base directory verified");

    let mut collect_actors = Vec::new();
    let mut built = true;
    for object in config["actors"].members() {

        let mut fmt = Actor {
            id:String::new(),
            signature:String::new(),
            port:0,
            r#type:String::new()
        };

        match object["id"].as_str() {
            Some(r) => {fmt.id = r.to_string();},
            None=>{built = false;common::error("failed-parse_actor-id");break;}
        };
        match object["signature"].as_str() {
            Some(r) => {fmt.signature = r.to_string();},
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

        collect_actors.push(fmt);

    }

    if built == false {
        return Err(common::error("failed-parse_actors"));
    }

    let app_dir = io::app_dir();
    let mut started = true;
    for a in &collect_actors {
        match start_actor(a,&secure,&base_dir,&app_dir) {
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

    Ok(collect_actors)

}

fn start_actor(actor:&Actor,secure:&String,base_dir:&String,app_dir:&String) -> Result<(),String> {

    //println!("actor : {:?}",actor);

    if actor.r#type == "files".to_string() {

        println!("app_dir : {:?}",&app_dir);

    }

    Ok(())

}
