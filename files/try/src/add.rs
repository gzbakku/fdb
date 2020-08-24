use postoffice::client::channel;
use json::{JsonValue};
use futures::executor::block_on;

pub fn init(){

    if false{
        add_only_once("23".to_string());
    }

    if false{
        add_only_once("1".to_string());
        add_only_once("2".to_string());
        add_only_once("3".to_string());
    }

    if true{
        add_file_once();
    }

}

fn add_file_once(){
    let mut req = JsonValue::new_object();
    req.insert("type","add_file").unwrap();
    let mut data = JsonValue::new_object();
    data.insert("file_name","7csf78d7sf89789709dsf8d8989").unwrap();
    data.insert("file_type","list").unwrap();
    data.insert("data",build_native_warehouse_file()).unwrap();
    req.insert("data",data).unwrap();
    send_request(&req);
}

fn add_only_once(index:String){
    let mut req = JsonValue::new_object();
    req.insert("type","add_item").unwrap();
    let mut data = JsonValue::new_object();
    data.insert("file_name","8dsf78d7sf89789709dsf8d8989").unwrap();
    data.insert("file_type","list").unwrap();
    data.insert("item_index",index).unwrap();
    data.insert("item_value",make_random_data().clone()).unwrap();
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

fn build_native_warehouse_file() -> String{
    let mut build = String::new();
    for i in 1..10 {
        let make_file = format!("{}++==++{}||--||",i,make_random_data());
        build.push_str(&make_file);
    }
    return build;
}

pub fn make_random_data() -> String{
    let mut data = JsonValue::new_object();
    data.insert("name","akku").unwrap();
    data.insert("age","69").unwrap();
    data.insert("email","akku@nope.com").unwrap();
    return data.dump();
}
