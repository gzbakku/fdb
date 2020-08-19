use json::JsonValue;
use postoffice::common;
use postoffice::client::send_message;

pub fn init(connection_id:&String){

    let mut request = JsonValue::new_object();
    request.insert("type","add").unwrap();

    let mut data = JsonValue::new_object();
    data.insert("index",common::hash::md5(&"1".to_string())).unwrap();
    data.insert("value",make_data()).unwrap();
    data.insert("dir","kola").unwrap();

    request.insert("data",data).unwrap();

    process_request(&connection_id,&request.dump(),false);

}

pub fn process_request(connection_id:&String,message:&String,secure:bool){
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
