use postoffice::client::channel;
use crate::formats::Act;
use json::JsonValue;
use futures::executor::block_on;
use postoffice::resp;
use crate::book;

pub fn init(act:&Act) -> Result<(),&'static str> {

    let mut data = JsonValue::new_object();
    data.insert("index",act.index.clone()).unwrap();
    data.insert("value",act.value.clone()).unwrap();

    let mut req = JsonValue::new_object();
    req.insert("type","add").unwrap();
    req.insert("data",data).unwrap();

    match block_on(channel::send(&"memBank".to_string(), &req, true)){
        Ok(response)=>{
            if !resp::is_successfull(&response.data){
                return Err("failed-add_to_mem_storage-init-add");
            }
            match book::add(act.index.clone(), response.name.clone()){
                Ok(_)=>{},
                Err(e)=>{
                    println!("failed-add_index_to_book-init-add => {:?}",e);
                }
            }
            return Ok(());
        },
        Err(_)=>{
            return Err("failed-send_as_round_robin-init-add");
        }
    }

}
