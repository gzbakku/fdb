use postoffice::client::channel;
use json::{JsonValue};
use futures::executor::block_on;

pub fn init(){

    if false{
        get_only_once();
    }
    if true{
        get_file_once();
    }
    if false{
        get_range_once();
    }
    if false{
        get_items_once(vec!["1","2","3"]);
    }

}

fn get_file_once(){
    let mut req = JsonValue::new_object();
    req.insert("type","get_file").unwrap();
    let mut data = JsonValue::new_object();
    data.insert("file_name","8dsf78d7sf89789709dsf8d8989").unwrap();
    data.insert("file_type","list").unwrap();
    req.insert("data",data).unwrap();
    send_request(&req);
}

fn get_range_once(){
    let mut req = JsonValue::new_object();
    req.insert("type","get_range").unwrap();
    let mut data = JsonValue::new_object();
    data.insert("file_name","8dsf78d7sf89789709dsf8d8989").unwrap();
    data.insert("file_type","list").unwrap();
    data.insert("start_index","1").unwrap();
    data.insert("end_index","5").unwrap();
    req.insert("data",data).unwrap();
    send_request(&req);
}

fn get_items_once(items:Vec<&'static str>){
    let mut req = JsonValue::new_object();
    req.insert("type","get_items").unwrap();
    let mut data = JsonValue::new_object();
    data.insert("file_name","8dsf78d7sf89789709dsf8d8989").unwrap();
    data.insert("file_type","list").unwrap();
    data.insert("items",items).unwrap();
    req.insert("data",data).unwrap();
    send_request(&req);
}

fn get_only_once(){
    let mut req = JsonValue::new_object();
    req.insert("type","get_item").unwrap();
    let mut data = JsonValue::new_object();
    data.insert("file_name","8dsf78d7sf89789709dsf8d8989").unwrap();
    data.insert("file_type","list").unwrap();
    data.insert("item_index","23").unwrap();
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
