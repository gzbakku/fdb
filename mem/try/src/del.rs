use json::JsonValue;
use postoffice::common;
use postoffice::client::send_message;
use postoffice::client::channel::{brodcast,send,send_to_member};
use futures::executor::block_on;

pub fn init(channel_name:&String){

    let mut request = JsonValue::new_object();
    request.insert("type","delete").unwrap();

    let mut data = JsonValue::new_object();
    data.insert("index",common::hash::md5(&"1".to_string())).unwrap();

    request.insert("data",data).unwrap();

    for _ in 0..1{
        process_brodcast(&channel_name,&request,false);
    }

}

fn process_brodcast(channel_name:&String,message:&JsonValue,secure:bool) -> Result<(),&'static str>{

    let run = block_on(brodcast(&channel_name, message, secure));
    match run{
        Ok(resp)=>{
            println!("{:?}",resp);
            return Ok(());
        },
        Err(e)=>{
            println!("failed brodcast message {:?}",e);
            return Err("failed-send_round_robin");
        }
    }

}

pub fn process_request_simple(connection_id:&String,message:&String,secure:bool){
    match send_message(&connection_id, message.to_string(), secure){
        Ok(resp)=>{
            println!("{:?}",resp.message);
        },
        Err(_)=>{
            println!("failed-process-request-add");
        }
    }
}

pub fn make_data() -> String{
    let mut data = JsonValue::new_object();
    data.insert("name","akku").unwrap();
    data.insert("age","69").unwrap();
    data.insert("email","akku@nope.com").unwrap();
    return data.dump();
}
