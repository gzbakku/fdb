use postoffice::{server,resp,collector};
use json::JsonValue;
use lazy_static::lazy_static;
use std::sync::Mutex;

mod path;
mod collect;
mod validate;
mod engine;
mod collections;
mod io;

struct BaseDir {
    path:String
}

impl BaseDir {
    fn overtake(self:&mut Self,path:String){
        self.path = path;
    }
}

lazy_static! {
    static ref BASE_DIR:Mutex<BaseDir> = Mutex::new(BaseDir {
        path:String::new()
    });
}

fn main() {

    let key = "8cfb30b34977529853bbe46afdbbd5ae".to_string();
    let address = String::from("127.0.0.1:5200");

    let cwd = collector::io::cwd();

    let base_dir =  format!("{}/instance",cwd);
    if !collector::io::ensure_dir(&base_dir) {
        println!("!!! failed-ensure_base_dir-for_collector");
        return;
    }

    match BASE_DIR.lock() {
        Ok(mut dir)=>{
            dir.overtake(base_dir.clone());
        },
        Err(_)=>{
            println!("!!! failed-lock_base_dir_mutex");
        }
    }

    let collections_dir = format!("{}/collections",&base_dir);
    if !collector::io::ensure_dir(&collections_dir) {
        println!("!!! failed-ensure_collections_dir-for_collector");
        return;
    }

    let collector_dir = format!("{}/collector",&base_dir);
    if !collector::io::ensure_dir(&collector_dir) {
        println!("!!! failed-ensure_collector_dir-for_collector");
        return;
    }

    if false {
        match collect::process_collections(&base_dir) {
            Ok(_)=>{},
            Err(_)=>{
                println!("failed init collector");
                return;
            }
        }
    }

    server::init(address,key,handler,auth);

}


fn auth(_token:server::auth::Token) -> bool {
    true
}

fn handler(req: server::Request) -> Result<server::Response,String> {

    let body:JsonValue;
    match validate::request(req.clone()) {
        Ok(parsed)=>{
            body = parsed;
        },
        Err(e)=>{
            let error = format!("check request failed error : {:?}",e);
            return Ok(resp::error(req,error));
        }
    }

    match body["type"].as_str() {
        Some(req_type)=>{
            if req_type == "read" {
                return Ok(engine::read::init(req,&body));
            } else if req_type == "write" {
                //test write engine directoly without writing data to collector
                if true {
                    let mut vec = Vec::new();
                    &vec.push(body);
                    match engine::write::init(vec) {
                        Ok(_)=>{
                            println!(">>> write engine successfull");
                        },
                        Err(e)=>{
                            let error = format!("failed-write_engine=>{}",e);
                            return Ok(resp::error(req,error));
                        }
                    }
                } else {
                    match collector::insert(&body.dump()) {
                        Ok(_)=>{},
                        Err(e)=>{
                            return Ok(resp::error(req,"failed-collect_data".to_string()));
                        }
                    }
                }
            } else {
                return Ok(resp::error(req,"failed-invalid_req_type".to_string()));
            }
        },
        None=>{
            return Ok(resp::error(req,"failed-extract_req_type_as_str".to_string()));
        }
    }

    return Ok(resp::ok(req));

}
