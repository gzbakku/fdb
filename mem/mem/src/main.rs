use postoffice::{server,resp};
use postoffice::config;
use postoffice::client::channel;

mod formats;
mod book;
mod add;
mod check;
mod find;
mod get;
mod del;

/*

    -i=529f07b0b1d5d3df52b0440bad708090 -k=192.168.0.1 -p=5611 -t=2dc6bb40e73417bba878d3c8e3e08780 -c=192.168.0.1:5200 -n=5100 -d="d://workstation/expo/rust/fdb/data"

    cargo watch -x "run -- -i=529f07b0b1d5d3df52b0440bad708090 -k=192.168.0.1 -p=5611 -t=2dc6bb40e73417bba878d3c8e3e08780 -c=192.168.0.1:5200 -n=5100 -d='d://workstation/expo/rust/fdb/data'"

    cargo run -- -i="529f07b0b1d5d3df52b0440bad708090" -k="192.168.0.1" -p="5611" -t="2dc6bb40e73417bba878d3c8e3e08780" -c="192.168.0.1:5200" -n="5100" -d="d://workstation/expo/rust/fdb/data"


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

    let addr_one = "127.0.0.1:5601".to_string();
    match channel::add_member(&"memBank".to_string(), &"one".to_string(), &addr_one, &this_config.session_token){
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
    match formats::parse_request(&r.data){
        Ok(a)=>{
            act = a;
        },
        Err(e)=>{
            println!("{:?}",e);
            return Ok(resp::error(r, "failed-parse-request".to_string()));
        }
    }

    if act.func == "add"{
        match add::init(&act){
            Ok(())=>{
                return Ok(resp::ok(r));
            },
            Err(e)=>{
                println!("!!! fdb_mem failed-add_item {:?}",e);
                return Ok(resp::error(r,"failed-add_item".to_string()));
            }
        }
    } else if act.func == "check"{
        match check::init(&act){
            Ok(data)=>{
                return Ok(resp::data(r,data,false));
            },
            Err(e)=>{
                println!("!!! fdb_mem failed-check_item {:?}",e);
                return Ok(resp::error(r,"failed-check_item".to_string()));
            }
        }
    } else if act.func == "get"{
        match get::init(&act){
            Ok(data)=>{
                return Ok(resp::data(r,data,true));
            },
            Err(e)=>{
                println!("!!! fdb_mem failed-get_item {:?}",e);
                return Ok(resp::error(r,"failed-get_item".to_string()));
            }
        }
    } else if act.func == "delete"{
        match del::init(&act){
            Ok(_)=>{
                return Ok(resp::ok(r));
            },
            Err(e)=>{
                println!("!!! fdb_mem failed-delete_item {:?}",e);
                return Ok(resp::error(r,"failed-delete_item".to_string()));
            }
        }
    }

    return Ok(resp::ok(r));

}
