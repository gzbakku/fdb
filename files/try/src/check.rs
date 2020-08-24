use postoffice::client::channel;
use json::{JsonValue};
use futures::executor::block_on;

pub fn init(){

    if true{
        add_only_one();
    }

}

fn add_only_one(){
    let mut req = JsonValue::new_object();
    req.insert("type","check_file").unwrap();
    let mut data = JsonValue::new_object();
    data.insert("file_name","8dsf78d7sf89789709dsf8d8989").unwrap();
    data.insert("file_type","list").unwrap();
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

pub fn make_random_data() -> String{
    let mut data = JsonValue::new_object();
    data.insert("name","akku").unwrap();
    data.insert("age","69").unwrap();
    data.insert("email","akku@nope.com").unwrap();
    return data.dump();
}
