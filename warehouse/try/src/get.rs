use postoffice::client::{send_message,common};
use json::{JsonValue,parse};

pub fn init(connection_id:&String){

    if false{
        get_index(&connection_id);
    }

    if true{
        get_values(&connection_id);
    }

}

pub fn get_values(connection_id:&String){

    let file_name = "e3089be6cecc5c3f5faac8140d84c92c-0_110".to_string();
    let file_type = "list".to_string();

    if true{
        get_item(&connection_id,&file_name,&file_type,"b1a3ffab3b4403037800555f2a776056".to_string());
        println!("\n");
    }

    if true{
        let mut items = Vec::new();
        items.push("b1a3ffab3b4403037800555f2a776056".to_string());
        items.push("b2b15e3b8c9cd29cbf2f0710a40c21a1".to_string());
        get_items(&connection_id,&file_name,&file_type,items);
        println!("\n");
    }

}

pub fn get_index(connection_id:&String){

    let file_name = "75a9b2778459040d25eda581a794fa0c-0_110".to_string();
    let file_type = "list".to_string();

    if true{
        get_item(&connection_id,&file_name,&file_type,"3".to_string());
        println!("\n");
    }
    if true{
        get_range(&connection_id,&file_name,&file_type);
        println!("\n");
    }
    if true{
        get_file(&connection_id,&file_name,&file_type);
        println!("\n");
    }

}

fn get_items(connection_id:&String,file_name:&String,file_type:&String,index:Vec<String>){
    let mut request = JsonValue::new_object();
    request.insert("type","get_items".to_string()).unwrap();
    let mut data = JsonValue::new_object();
    data.insert("file_name",file_name.clone()).unwrap();
    data.insert("file_type",file_type.clone()).unwrap();
    let mut docs = JsonValue::new_array();
    for doc in index{
        docs.push(doc).unwrap();
    }
    data.insert("items",docs).unwrap();
    request.insert("data",data).unwrap();
    process_message(&connection_id, &request);
}

fn get_item(connection_id:&String,file_name:&String,file_type:&String,index:String){
    let mut request = JsonValue::new_object();
    request.insert("type","get_item".to_string()).unwrap();
    let mut data = JsonValue::new_object();
    data.insert("file_name",file_name.clone()).unwrap();
    data.insert("file_type",file_type.clone()).unwrap();
    data.insert("item_index",index.to_string()).unwrap();
    request.insert("data",data).unwrap();
    process_message(&connection_id, &request);
}

fn get_range(connection_id:&String,file_name:&String,file_type:&String){
    let mut request = JsonValue::new_object();
    request.insert("type","get_range".to_string()).unwrap();
    let mut data = JsonValue::new_object();
    data.insert("file_name",file_name.clone()).unwrap();
    data.insert("file_type",file_type.clone()).unwrap();
    data.insert("start_index","3".to_string()).unwrap();
    data.insert("end_index","7".to_string()).unwrap();
    request.insert("data",data).unwrap();
    process_message(&connection_id, &request);
}

fn get_file(connection_id:&String,file_name:&String,file_type:&String){
    let mut request = JsonValue::new_object();
    request.insert("type","get_file".to_string()).unwrap();
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
