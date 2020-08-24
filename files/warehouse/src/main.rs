use postoffice::resp;
use postoffice::{server,collector,io};
use postoffice::server::{Request,Response};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;
use postoffice::config;

mod worker;
mod formats;

/*

    -i=529f07b0b1d5d3df52b0440bad708090 -k=192.168.0.1 -p=5701 -t=2dc6bb40e73417bba878d3c8e3e08780 -c=192.168.0.1:5200 -n=5100 -d="d://workstation/expo/rust/fdb/data"

    cargo watch -x "run -- -i=529f07b0b1d5d3df52b0440bad708090 -k=192.168.0.1 -p=5701 -t=2dc6bb40e73417bba878d3c8e3e08780 -c=192.168.0.1:5200 -n=5100 -d='d://workstation/expo/rust/fdb/data'"

    cargo run -- -i="529f07b0b1d5d3df52b0440bad708090" -k="192.168.0.1" -p="5701" -t="2dc6bb40e73417bba878d3c8e3e08780" -c="192.168.0.1:5200" -n="5100" -d="d://workstation/expo/rust/fdb/data"


*/

struct BaseDir{
    path:String
}

impl BaseDir{
    fn new() -> BaseDir{
        BaseDir{
            path:String::new()
        }
    }
}

struct FileLock{
    map:HashMap<String,String>
}

impl FileLock{
    fn new() -> FileLock{
        FileLock{
            map:HashMap::new()
        }
    }
}

lazy_static! {
    static ref BASE_DIR: Mutex<BaseDir> = Mutex::new(BaseDir::new());
    static ref FILE_LOCK: Mutex<FileLock> = Mutex::new(FileLock::new());
}

fn main(){

    let this_dir = "d://workstation/expo/rust/fdb";
    // let base_dir = format!("{}/vault/warehouse/",this_dir);
    let files_dir = format!("{}/vault/warehouse/files",this_dir);
    let collector_dir = format!("{}/vault/warehouse/collector/",this_dir);

    match BASE_DIR.lock(){
        Ok(mut lock)=>{
            lock.path = files_dir;
        },
        Err(_)=>{
            panic!("failed-lock_base_dir");
        }
    }

    io::ensure_dir(&collector_dir);
    match collector::init(collector_dir){
        Ok(_)=>{
            worker::init();
        },
        Err(e)=>{
            println!("collector failed to intiate : {:?}",e);
        }
    }

    // let key = "8cfb30b34977529853bbe46afdbbd5ae".to_string();
    // let address = String::from("127.0.0.1:5701");
    // server::init(address,key,handler,auth);

    let this_config:config::Config;
    match config::init("warehouse","0.1","warehouse worker for fdb"){
        Ok(c)=>{
            this_config = c;
        },
        Err(e)=>{
            println!("failed get worker shiled data : {:?}",e);
            return;
        }
    }

    let address = format!("127.0.0.1:{}",this_config.worker_port);
    server::init(address,this_config.session_token,handler,auth);

}

fn auth(_:server::auth::Token) -> bool {
    //println!("token : {:?}",token);
    return true;
}

fn handler(req: Request) -> Result<Response,String> {

    let act:formats::Act;
    match formats::parse_activity(&req.data){
        Ok(a)=>{
            act = a;
        },
        Err(e)=>{
            println!("!!! invalid_request : {:?}",e);
            return Ok(resp::error(req,"invalid_request".to_string()));
        }
    }

    if act.func == "get_item"{
        match worker::get::item::init(act){
            Ok(data)=>{
                return Ok(resp::data(req, data, false));
            },
            Err(_)=>{
                return Ok(resp::error(req,"failed-get_item".to_string()));
            }
        }
    } else if act.func == "get_items"{
        match worker::get::items::init(act){
            Ok(data)=>{
                return Ok(resp::data(req, data, false));
            },
            Err(_)=>{
                return Ok(resp::error(req,"failed-get_items".to_string()));
            }
        }
    } else if act.func == "get_range"{
        match worker::get::range::init(act){
            Ok(data)=>{
                return Ok(resp::data(req, data, false));
            },
            Err(_)=>{
                return Ok(resp::error(req,"failed-get_range".to_string()));
            }
        }
    } else if act.func == "get_file"{
        match worker::get::file::init(act){
            Ok(data)=>{
                return Ok(resp::data(req, data, false));
            },
            Err(_)=>{
                return Ok(resp::error(req,"failed-get_range".to_string()));
            }
        }
    } else if act.func == "list_dir"{
        match worker::list::init(act){
            Ok(data)=>{
                return Ok(resp::data(req, data, false));
            },
            Err(_)=>{
                return Ok(resp::error(req,"failed-list_dir".to_string()));
            }
        }
    } else if act.func == "check_file"{
        match worker::get::check::init(act){
            Ok(data)=>{
                return Ok(resp::data(req, data, false));
            },
            Err(_)=>{
                return Ok(resp::error(req,"failed-check_file".to_string()));
            }
        }
    } else {
        match collector::insert(&req.data){
            Ok(_)=>{},
            Err(e)=>{
                let error = format!("que request failed error : {:?}",e);
                return Ok(resp::error(req,error));
            }
        }
    }

    return Ok(resp::ok(req));

}
