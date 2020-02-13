use postoffice::{client,resp};
use json;
mod io;

fn main() {

    let cwd = io::cwd();
    let path = format!("{}\\sample_store_data.json",cwd);

    match io::read(path) {
        Ok(json)=>{
            connect(json);
        },
        Err(e)=>{
            println!("failed to read the json file error : {:?}",e);
        }
    }

}

fn connect(data:json::JsonValue){

    let connection_id = client::get_random_connection_id();
    let addr = "127.0.0.1:5200".to_string();
    let key = "8cfb30b34977529853bbe46afdbbd5ae".to_string();

    match client::start_connection(&connection_id,addr,key) {
        Ok(_)=>{
            send(connection_id,data);
        },
        Err(_)=>{
            println!("!!! failed to start connection");
        }
    }

}

fn send(connection_id:String,data:json::JsonValue){

    let mut request_object = json::JsonValue::new_object();

    match request_object.insert("type","write") {
        Ok(_)=>{},
        Err(_)=>{}
    }

    match request_object.insert("data",data) {
        Ok(_)=>{},
        Err(_)=>{}
    }

    let request_string = request_object.dump();

    match client::send_message(&connection_id,request_string,false) {
        Ok(response)=>{

            match resp::parse_response(response) {
                Ok(result)=>{

                    println!("result : {:?}",result);

                },
                Err(e)=>{
                    println!("error parse response strucft : {:?}",e);
                }
            }

        },
        Err(_)=>{
            println!("request failed");
        }
    }

}
