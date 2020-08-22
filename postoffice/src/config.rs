extern crate clap;
use clap::{Arg, App};

/*

    -i=529f07b0b1d5d3df52b0440bad708090 -k=192.168.0.1 -p=5600 -t=2dc6bb40e73417bba878d3c8e3e08780 -c=192.168.0.1:5200 -n=5100 -d="d://workstation/expo/rust/fdb/data"

    cargo watch -x "run -- -i=529f07b0b1d5d3df52b0440bad708090 -k=192.168.0.1 -p=5600 -t=2dc6bb40e73417bba878d3c8e3e08780 -c=192.168.0.1:5200 -n=5100 -d='d://workstation/expo/rust/fdb/data'"

    cargo run -- -i="529f07b0b1d5d3df52b0440bad708090" -k="192.168.0.1" -p="5600" -t="2dc6bb40e73417bba878d3c8e3e08780" -c="192.168.0.1:5200" -n="5100" -d="d://workstation/expo/rust/fdb/data"


*/

#[derive(Debug,Clone)]
pub struct Config{
    pub worker_id:String,               //i
    pub worker_ip:String,               //k
    pub worker_port:String,             //p
    pub session_token:String,           //t
    pub composer_address:String,        //c
    pub node_port:String,               //n
    pub base_dir:String                 //d
}

pub fn init(app_name:&str,version:&str,description:&str) -> Result<Config,&'static str>{

    //a worker takes uniqid, session token, composer address and node port.

    let matches = App::new(app_name)
        .author("tejasav dutt. <gzbakku@gmail.com>")
        .version(version)
        .about(description)
        .arg(Arg::with_name("worker_id")
             .short("i")
             .long("worker_id")
             .value_name("worker_id")
             .help("uniqid of the worker")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("worker_ip")
             .short("k")
             .long("worker_ip")
             .value_name("worker_ip")
             .help("ip of the worker")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("worker_port")
             .short("p")
             .long("worker_port")
             .value_name("worker_port")
             .help("port of the worker")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("session_token")
            .short("t")
            .long("session_token")
            .value_name("session_token")
            .help("session token received from composer.")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("composer_address")
            .short("c")
            .long("composer_address")
            .value_name("composer_address")
            .help("composer address with port")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("node_port")
            .short("n")
            .long("node_port")
            .value_name("node_port")
            .help("port on which node is listening on.")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("base_dir")
            .short("d")
            .long("base_dir")
            .value_name("base_dir")
            .help("base dir to store files in.")
            .required(true)
            .takes_value(true))
        .get_matches();


    let worker_id:String;
    match matches.value_of("worker_id"){
        Some(v)=>{
            worker_id = v.to_string();
        },
        None=>{
            return Err("failed-extract-worker_id");
        }
    }

    let worker_ip:String;
    match matches.value_of("worker_ip"){
        Some(v)=>{
            worker_ip = v.to_string();
        },
        None=>{
            return Err("failed-extract-worker_ip");
        }
    }

    let worker_port:String;
    match matches.value_of("worker_port"){
        Some(v)=>{
            worker_port = v.to_string();
        },
        None=>{
            return Err("failed-extract-worker_port");
        }
    }

    let session_token:String;
    match matches.value_of("session_token"){
        Some(v)=>{
            session_token = v.to_string();
        },
        None=>{
            return Err("failed-extract-session_token");
        }
    }

    let composer_address:String;
    match matches.value_of("composer_address"){
        Some(v)=>{
            composer_address = v.to_string();
        },
        None=>{
            return Err("failed-extract-composer_address");
        }
    }

    let node_port:String;
    match matches.value_of("node_port"){
        Some(v)=>{
            node_port = v.to_string();
        },
        None=>{
            return Err("failed-extract-node_port");
        }
    }

    let base_dir:String;
    match matches.value_of("base_dir"){
        Some(v)=>{
            base_dir = v.to_string();
        },
        None=>{
            return Err("failed-extract-base_dir");
        }
    }

    return Ok(Config {
        worker_id:worker_id,
        worker_ip:worker_ip,
        worker_port:worker_port,
        session_token:session_token,
        composer_address:composer_address,
        node_port:node_port,
        base_dir:base_dir,
    });

}
