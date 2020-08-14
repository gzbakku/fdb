use postoffice::client::{send_message,common};
use json::{JsonValue,parse};

pub fn init(connection_id:&String){

    let file_name = "0ae0106ae1aa177baf3cc8a91b3a1f1f-0_110".to_string();
    let file_type = "list".to_string();

    if true{
        delete_item(&connection_id,&file_name,&file_type);
        println!("\n");
    }
    if false{
        delete_file(&connection_id,&file_name,&file_type);
        println!("\n");
    }

}

fn delete_item(connection_id:&String,file_name:&String,file_type:&String){
    let mut request = JsonValue::new_object();
    request.insert("type","delete_item".to_string()).unwrap();
    let mut data = JsonValue::new_object();
    data.insert("file_name",file_name.clone()).unwrap();
    data.insert("file_type",file_type.clone()).unwrap();
    data.insert("item_index","3".to_string()).unwrap();
    request.insert("data",data).unwrap();
    process_message(&connection_id, &request);
}

fn delete_file(connection_id:&String,file_name:&String,file_type:&String){
    let mut request = JsonValue::new_object();
    request.insert("type","delete_file".to_string()).unwrap();
    let mut data = JsonValue::new_object();
    data.insert("file_name",file_name.clone()).unwrap();
    data.insert("file_type",file_type.clone()).unwrap();
    request.insert("data",data).unwrap();
    process_message(&connection_id, &request);
}

fn process_message(connection_id:&String,msg:&JsonValue){
    match send_message(&connection_id, msg.dump(), false) {
        Ok(response)=>{
            if response.result == true{
                match parse(&response.message){
                    Ok(r)=>{
                        println!("{}",r);
                    },
                    Err(_)=>{
                        println!("invalid response");
                    }
                }
            }
        },
        Err(_)=>{
            common::error("request-failed");
        }
    }
}
