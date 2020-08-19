
use postoffice::client::{start_connection,get_random_connection_id,common,send_message};

fn main() {

    let key = "0554ac53f239c96279a3cff5cb29b085".to_string();
    let address = String::from("127.0.0.1:5201");
    let connection_id = get_random_connection_id();

    match start_connection(&connection_id,address,key) {
        Ok(_)=>{
            println!("connection established");
        },
        Err(_)=>{
            common::error("failed start connection");
        }
    }

    match send_message(&connection_id, "hello".to_string(), false){
        Ok(r)=>{
            match r.parse_to_json(){
                Ok(json)=>{
                    println!("{:?}",json);
                },
                Err(_)=>{
                    println!("failed parse response to json");
                }
            }
        },
        Err(e)=>{
            println!("failed send messgae : {:?}",e);
        }
    }

}
