use postoffice::{io};
use postoffice::client::{send_message_async,common,send_message,send_message_sync};
use json::{JsonValue,parse};
use std::fs::read_to_string;
use futures::future::join_all;
use futures::executor::block_on;
use std::pin::Pin;
use futures::future::BoxFuture;
use futures::future::Future;
use tokio;

pub fn init(connection_id:&String){

    block_on(start_adding(connection_id.clone()));

}

async fn start_adding(connection_id:String) {

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
                    // return Err("failed-parse-data_file");
                }
            }
        },
        Err(e)=>{
            let error = format!("failed-read_test_data : {:?}",e);
            println!("{}",error);
            return;
            // return Err("failed-read-data_file");
        }
    }

    let mut collect: Vec<BoxFuture<Result<(),&'static str>>> = Vec::new();

    if true{
        for i in 0..test_data.len(){
            collect.push(
                Box::pin(process_message(connection_id.clone(), test_data[i].clone(), i))
            );
        }
    }

    if false{
        for i in 0..1{
            collect.push(
                Box::pin(process_message(connection_id.clone(), test_data[i].clone(), i))
            );
        }
    }

    let work = join_all(collect).await;

    for resp in work{
        println!("{:?}",resp);
    }

}

async fn process_message(connection_id:String,msg:JsonValue,i:usize) -> Result<(),&'static str>{

    let run = send_message_sync(&connection_id, msg.dump(), false);

    match run {
        Ok(response)=>{
            if response.result == true{
                match parse(&response.message){
                    Ok(r)=>{
                        println!("{} : {}",r,i);
                        return Ok(());
                    },
                    Err(_)=>{
                        println!("invalid response");
                        return Err("failed-parse-response");
                    }
                }
            } else {
                return Err("failed-invalid-response");
            }
        },
        Err(_)=>{
            common::error("request-failed");
            return Err("failed-request");
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
