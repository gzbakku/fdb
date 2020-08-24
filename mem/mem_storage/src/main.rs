use postoffice::{server,resp};
use postoffice::config;

mod formats;
mod book;
mod get;
mod check;

/*

    -i=529f07b0b1d5d3df52b0440bad708090 -k=192.168.0.1 -p=5602 -t=2dc6bb40e73417bba878d3c8e3e08780 -c=192.168.0.1:5200 -n=5100 -d="d://workstation/expo/rust/fdb/data"

    cargo watch -x "run -- -i=529f07b0b1d5d3df52b0440bad708090 -k=192.168.0.1 -p=5602 -t=2dc6bb40e73417bba878d3c8e3e08780 -c=192.168.0.1:5200 -n=5100 -d='d://workstation/expo/rust/fdb/data'"

    cargo run -- -i="529f07b0b1d5d3df52b0440bad708090" -k="192.168.0.1" -p="5602" -t="2dc6bb40e73417bba878d3c8e3e08780" -c="192.168.0.1:5200" -n="5100" -d="d://workstation/expo/rust/fdb/data"


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

    let address = format!("127.0.0.1:{}",this_config.worker_port);
    server::init(address,this_config.session_token,handler,auth);

}

fn auth(_:server::auth::Token) -> bool {
    //println!("token : {:?}",token);
    return true;
}

fn handler(r:server::Request) -> Result<server::Response,String>{

    let act:formats::Act;
    match formats::parse_request(&r.data){
        Ok(a)=>{
            act = a;
        },
        Err(e)=>{
            println!("!!! mem_storage failed-parse-request : {:?}",e);
            return Ok(resp::error(r, "failed-parse-request".to_string()));
        }
    }

    if act.func == "add"{
        match book::add(act.index, act.value){
            Ok(_)=>{
                return Ok(resp::ok(r));
            },
            Err(_)=>{
                return Ok(resp::error(r,"failed-add_item".to_string()));
            }
        }
    } else if act.func == "get"{
        match get::init(&act.index){
            Ok(obj)=>{
                return Ok(resp::data(r,obj,false));
            },
            Err(_)=>{
                return Ok(resp::error(r,"failed-get_item".to_string()));
            }
        }
    } else if act.func == "delete"{
        match book::remove(act.index){
            Ok(_)=>{
                return Ok(resp::ok(r));
            },
            Err(_)=>{
                return Ok(resp::error(r,"failed-remove_item".to_string()));
            }
        }
    } else if act.func == "check"{
        match check::init(&act.index){
            Ok(data)=>{
                return Ok(resp::data(r,data,false));
            },
            Err(_)=>{
                return Ok(resp::error(r,"failed-check_item".to_string()));
            }
        }
    } else {
        return Ok(resp::error(r,"failed-parse_request".to_string()));
    }

}
