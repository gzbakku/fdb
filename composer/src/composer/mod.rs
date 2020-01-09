use crate::io::Extracted;
use crate::common;

mod server;

pub fn init(e:&Extracted){

    common::log("initiating composer");

    //start the central server
    match crate::starter::init(&e) {
        Ok(actors)=>{
            for actor in actors {
                if actor.r#type == "composer".to_string(){
                    server::init(actor,e);
                }
            }
        },
        Err(_)=>{
            common::error("failed-init-starter");
            std::process::exit(1);
        }
    }

    //start the actors
    //

}
