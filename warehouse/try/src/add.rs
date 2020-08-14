use postoffice::{io};
use postoffice::client::{send_message,common};
use json::{JsonValue,parse};
use std::fs::read_to_string;

pub fn init(connection_id:&String){


    //read test data
    let test_data_path = format!("{}/warehouse.json",io::cwd());
    let test_data:JsonValue;
    match read_to_string(test_data_path){
        Ok(str)=>{
            match parse(&str){
                Ok(obj)=>{
                    test_data = obj;
                },
                Err(e)=>{
                    let error = format!("failed-parse_test_data : {:?}",e);
                    println!("{}",error);
                    return;
                }
            }
        },
        Err(e)=>{
            let error = format!("failed-read_test_data : {:?}",e);
            println!("{}",error);
            return;
        }
    }

    if true{
        for i in 0..test_data.len(){
            process_message(&connection_id, &test_data[i], i);
        }
    }

    if false{
        for i in 0..1{
            process_message(&connection_id, &test_data[i], i);
        }
    }

}

fn process_message(connection_id:&String,msg:&JsonValue,i:usize){
    match send_message(&connection_id, msg.dump(), false) {
        Ok(response)=>{
            if response.result == true{
                match parse(&response.message){
                    Ok(r)=>{
                        println!("{} : {}",r,i);
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

// fn add_item_request() -> String {
//     let mut message = JsonValue::new_object();
//     message.insert("type","add_item").unwrap();
//     let mut data = JsonValue::new_object();
//     data.insert("file_name","8dsf78d7sf897897_0-500").unwrap();
//     data.insert("file_type","list").unwrap();
//     data.insert("item_index","23").unwrap();
//     data.insert("item_value","dsf87897897sdf").unwrap();
//     message.insert("data",data).unwrap();
//     return message.dump();
// }
