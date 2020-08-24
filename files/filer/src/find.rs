use postoffice::check::{Format,Field,check};
use postoffice::resp::{is_successfull,with_data};
use postoffice::client::channel;
use json::JsonValue;
use crate::book;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;
use mio::{Events, Token, Waker, Poll};
use std::thread;
use std::time::Duration;
use futures::executor::block_on;

lazy_static! {
    static ref FOUNDERS : Mutex<HashMap<String,Vec<Waker>>> = Mutex::new(HashMap::new());
    static ref BRODCASTS : Mutex<HashMap<String,String>> = Mutex::new(HashMap::new());
}

//protocol

/*
    request comes in and asks for a founder
    founder is registered if not exists and inserts a brodcast request if is first in que
    brocast request runs in a async thread and every request wakes up its owners
*/

fn process_founder(file_name:String,file_type:String) -> Result<(),&'static str>{

    let file_anchor = format!("{}_{}",file_name,file_type);

    //make waker
    let mut poll:Poll;
    match Poll::new(){
        Ok(o)=>{
            poll = o;
        },
        Err(_)=>{
            return Err("failed-start_poll-process_founder");
        }
    }
    let mut events = Events::with_capacity(2);
    const WAKE_TOKEN: Token = Token(10);
    let waker:Waker;
    match Waker::new(poll.registry(), WAKE_TOKEN){
        Ok(w)=>{
            waker = w;
        },
        Err(_)=>{
            return Err("failed-start_waker-process_founder");
        }
    }

    match FOUNDERS.lock(){
        Ok(mut lock)=>{
            if !lock.contains_key(&file_anchor.to_string()){
                match BRODCASTS.lock(){
                    Ok(mut pool)=>{
                        if !pool.contains_key(&file_name){
                            match pool.insert(file_name,file_type){
                                Some(_)=>{},
                                None=>{}
                            }
                        }
                    },
                    Err(_)=>{
                        return Err("failed-lock-BRODCASTS-process_founder");
                    }
                }
                lock.insert(file_anchor.clone(),vec![waker]);
            } else {
                match lock.get_mut(&file_anchor.clone()){
                    Some(pool)=>{
                        pool.push(waker);
                    },
                    None=>{
                        return Err("failed-get_pool-process_founder");
                    }
                }
            }
        },
        Err(_)=>{
            return Err("failed-lock-FOUNDERS");
        }
    }

    match poll.poll(&mut events, None){
        Ok(_)=>{
            return Ok(());
        },
        Err(_)=>{
            return Err("failed-poll-find_brodcast");
        }
    }

}

pub fn init(file_name:String,file_type:String) -> Result<bool,&'static str>{
    match process_founder(file_name.clone(),file_type.clone()){
        Ok(_)=>{},
        Err(_)=>{
            return Err("failed-process_founder-init");
        }
    }
    let file_anchor = format!("{}_{}",file_name,file_type);
    match book::check(file_anchor.clone()){
        Ok(found)=>{
            return Ok(found);
        },
        Err(_)=>{
            return Err("failed-check_local_book-init");
        }
    }
}

pub fn start(){
    thread::spawn(|| {
        let sleep = Duration::from_millis(10);
        loop{
            match BRODCASTS.lock(){
                Ok(mut lock)=>{
                    if lock.len() == 0{
                        thread::sleep(sleep);
                    } else {
                        let hold = lock.clone();
                        match process_brodcasts(hold){
                            Ok(_)=>{
                                lock.clear();
                            },
                            Err(_)=>{
                                println!("failed-process_brodcasts-loop-start");
                                thread::sleep(sleep);
                            }
                        }
                    }
                },
                Err(_)=>{
                    println!("failed-get_lock-brodcasts-loop-start");
                    thread::sleep(sleep);
                }
            }
        }
    });
}

fn process_brodcasts(pool:HashMap<String,String>) -> Result<(),&'static str>{
    for file_name in pool.keys(){
        match pool.get(file_name){
            Some(file_type)=>{
                match process_find_index(file_name.clone(),file_type.clone()){
                    Ok(_)=>{},
                    Err(_)=>{}
                }
            },
            None=>{}
        }
    }
    return Ok(());
}

pub fn process_find_index(file_name:String,file_type:String) -> Result<(),&'static str>{
    let file_anchor = format!("{}_{}",file_name,file_type);
    let mut req = JsonValue::new_object();
    req.insert("type","check_file").unwrap();
    let mut data = JsonValue::new_object();
    data.insert("file_name",file_name).unwrap();
    data.insert("file_type",file_type).unwrap();
    req.insert("data",data).unwrap();
    let run = block_on(channel::brodcast(&"warehouse".to_string(), &req, true));
    // let run = channel::brodcast(&"memBank".to_string(), &req, true).await;
    match run{
        Ok(responses)=>{
            match parse_brocast_check(responses,file_anchor.clone()){
                Ok(_)=>{
                    match FOUNDERS.lock(){
                        Ok(mut lock)=>{
                            match lock.get(&file_anchor.clone()){
                                Some(pool)=>{
                                    for waker in pool{
                                        match waker.wake(){
                                            Ok(_)=>{},
                                            Err(_)=>{
                                                return Err("failed-waker_a_waker-process_find_index-find");
                                            }
                                        }
                                    }
                                },
                                None=>{
                                    return Err("failed-get_index_wakers-process_find_index-find");
                                }
                            }
                            match lock.remove(&file_anchor.clone()){
                                Some(_)=>{
                                    return Ok(());
                                },
                                None=>{
                                    return Err("failed-clear_all_wakers-process_find_index-find");
                                }
                            }
                        },
                        Err(_)=>{
                            return Err("failed-lock_founders-process_find_index-find");
                        }
                    }
                },
                Err(_)=>{
                    return Err("failed-parse_brodcast-process_find_index-find");
                }
            }
        },
        Err(_)=>{
            return Err("failed-brodcast_check_request-process_find_index-find");
        }
    }
}

fn parse_brocast_check(pool:Vec<channel::ChannelResponse>,file_anchor:String) -> Result<bool,&'static str>{
    for response in pool{
        match check_response(response){
            Ok(result)=>{
                if result.exists{
                    match book::add(file_anchor.clone(), result.name){
                        Ok(_)=>{
                            return Ok(true);
                        },
                        Err(e)=>{
                            println!("failed-add_index_to_book-parse_brocast_check-init-find => {:?}",e);
                        }
                    }
                }
            },
            Err(e)=>{
                println!("failed-check_response-parse_brocast_check-init-find => {:?}",e);
            }
        }
    }
    return Ok(false);
}

struct CheckChannelResponse{
    name:String,
    exists:bool
}

impl CheckChannelResponse{
    fn build(e:bool,name:&String) -> CheckChannelResponse{
        CheckChannelResponse{
            name:name.to_string(),
            exists:e
        }
    }
}

fn check_response(response:channel::ChannelResponse) -> Result<CheckChannelResponse,&'static str>{
    if !is_successfull(&response.data) || !with_data(&response.data){
        return Err("invalid_or_failed_response-check_response-parse_brocast_check-init-find");
    }
    // println!("{:#?}",&response.data);
    // println!("{:#?}",&response.data.dump());
    match check(&response.data["data"], data_format()){
        Ok(_)=>{
            match response.data["data"]["found"].as_bool(){
                Some(found)=>{
                    return Ok(CheckChannelResponse::build(found, &response.name));
                },
                None=>{
                    return Err("invalid_data-check_response-parse_brocast_check-init-find");
                }
            }
        },
        Err(e)=>{
            println!("invalid_data-check_response-parse_brocast_check-init-find => {:?}",e);
            return Err("invalid_data-check_response-parse_brocast_check-init-find");
        }
    }
}

fn data_format() -> Format{
    Format::builder(vec![
        Field::new("bool",false,"found",Field::no_options(),Field::no_format(),0,0,false)
    ])
}
