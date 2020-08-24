use postoffice::{server,resp};
use postoffice::config;
use postoffice::client::channel;

mod formats;
mod book;
mod find;

mod workers;

/*

    -i=529f07b0b1d5d3df52b0440bad708090 -k=192.168.0.1 -p=5711 -t=2dc6bb40e73417bba878d3c8e3e08780 -c=192.168.0.1:5200 -n=5100 -d="d://workstation/expo/rust/fdb/data"

    cargo watch -x "run -- -i=529f07b0b1d5d3df52b0440bad708090 -k=192.168.0.1 -p=5711 -t=2dc6bb40e73417bba878d3c8e3e08780 -c=192.168.0.1:5200 -n=5100 -d='d://workstation/expo/rust/fdb/data'"

    cargo run -- -i="529f07b0b1d5d3df52b0440bad708090" -k="192.168.0.1" -p="5711" -t="2dc6bb40e73417bba878d3c8e3e08780" -c="192.168.0.1:5200" -n="5100" -d="d://workstation/expo/rust/fdb/data"


*/

fn main() {

    let this_config:config::Config;
    match config::init("mem","0.1","mem worker for fdb"){
        Ok(c)=>{
            this_config = c;
        },
        Err(e)=>{
            println!("failed get worker shiled data : {:?}",e);
            return;
        }
    }

    let addr_one = "127.0.0.1:5701".to_string();
    match channel::add_member(&"warehouse".to_string(), &"one".to_string(), &addr_one, &this_config.session_token){
        Ok(_)=>{},
        Err(e)=>{
            println!("failed-add_memeber_to_channel-{:?}",e);
            return;
        }
    }

    find::start();

    let address = format!("127.0.0.1:{}",this_config.worker_port);
    server::init(address,this_config.session_token,handler,auth);

}

fn auth(_:server::auth::Token) -> bool {
    //println!("token : {:?}",token);
    return true;
}

fn handler(r:server::Request) -> Result<server::Response,String>{

    let act:formats::Act;
    match formats::parse_activity(&r.data){
        Ok(a)=>{
            act = a;
        },
        Err(e)=>{
            println!("{:?}",e);
            return Ok(resp::error(r, "failed-parse-request".to_string()));
        }
    }

    if act.func == "add_item"{
        match workers::add_item::init(&act){
            Ok(_)=>{
                return Ok(resp::ok(r));
            },
            Err(_)=>{
                // println!("!!! failed-add_item => {:?}",e);
                return Ok(resp::error(r,"failed-add_item".to_string()));
            }
        }
    } else if act.func == "add_file"{
        match workers::add_file::init(&act){
            Ok(_)=>{
                return Ok(resp::ok(r));
            },
            Err(_)=>{
                // println!("!!! failed-add_item => {:?}",e);
                return Ok(resp::error(r,"failed-add_file".to_string()));
            }
        }
    } else if act.func == "get_item"{
        match workers::get_item::init(&act){
            Ok(data)=>{
                return Ok(resp::data(r,data,false));
            },
            Err(_)=>{
                return Ok(resp::error(r,"failed-get_item".to_string()));
            }
        }
    } else if act.func == "get_items"{
        match workers::get_items::init(&act){
            Ok(data)=>{
                return Ok(resp::data(r,data,false));
            },
            Err(_)=>{
                return Ok(resp::error(r,"failed-get_items".to_string()));
            }
        }
    } else if act.func == "get_range"{
        match workers::get_range::init(&act){
            Ok(data)=>{
                return Ok(resp::data(r,data,false));
            },
            Err(e)=>{
                println!("!!! failed-get_range => {:?}",e);
                return Ok(resp::error(r,"failed-get_range".to_string()));
            }
        }
    } else if act.func == "get_file"{
        match workers::get_file::init(&act){
            Ok(data)=>{
                return Ok(resp::data(r,data,false));
            },
            Err(e)=>{
                println!("!!! failed-get_file => {:?}",e);
                return Ok(resp::error(r,"failed-get_file".to_string()));
            }
        }
    } else if act.func == "check_file"{
        match workers::check_file::init(&act){
            Ok(data)=>{
                return Ok(resp::data(r,data,false));
            },
            Err(_)=>{
                return Ok(resp::error(r,"failed-check_file".to_string()));
            }
        }
    } else if act.func == "delete_item"{
        match workers::delete_item::init(&act){
            Ok(data)=>{
                return Ok(resp::ok(r));
            },
            Err(_)=>{
                return Ok(resp::error(r,"failed-delete_item".to_string()));
            }
        }
    } else if act.func == "delete_file"{
        match workers::delete_file::init(&act){
            Ok(data)=>{
                return Ok(resp::ok(r));
            },
            Err(_)=>{
                return Ok(resp::error(r,"failed-delete_file".to_string()));
            }
        }
    }

    return Ok(resp::ok(r));

}
