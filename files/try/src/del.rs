use postoffice::client::channel;
use json::{JsonValue};
use futures::executor::block_on;

pub fn init(){

    if true{
        delete_file_once();
    } else {
        delete_only_once("1".to_string());
    }

}

fn delete_file_once(){
    let mut req = JsonValue::new_object();
    req.insert("type","delete_file").unwrap();
    let mut data = JsonValue::new_object();
    data.insert("file_name","7csf78d7sf89789709dsf8d8989").unwrap();
    data.insert("file_type","list").unwrap();
    req.insert("data",data).unwrap();
    send_request(&req);
}

fn delete_only_once(index:String){
    let mut req = JsonValue::new_object();
    req.insert("type","delete_item").unwrap();
    let mut data = JsonValue::new_object();
    data.insert("file_name","7csf78d7sf89789709dsf8d8989").unwrap();
    data.insert("file_type","list").unwrap();
    data.insert("item_index",index).unwrap();
    req.insert("data",data).unwrap();
    send_request(&req);
}

fn send_request(message:&JsonValue){
    match block_on(channel::send(&"filer".to_string(), message, true)){
        Ok(resp)=>{
            println!("{:?}",resp);
        },
        Err(_)=>{
            println!("failed-add_request");
        }
    }
}
