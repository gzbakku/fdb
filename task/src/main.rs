use std::{thread,time};
use lazy_static::lazy_static;
use std::sync::Mutex;
use mio::{Events, Token, Waker, Poll};
use std::collections::HashMap;
use futures::future::{join_all,Future};
use futures::executor::block_on;

struct Responses{
    name:String
}

lazy_static! {
    #[derive(Debug)]
    static ref CHANNELS : Mutex<HashMap<String,Waker>> = Mutex::new(HashMap::new());
}

fn main(){
    println!("Hello, world!");
    block_on(run());
}

async fn run() {

    println!("hello run");

    let channel_name = "one".to_string();

    let waker = wakeup(channel_name.clone());
    let maker = make_channel(channel_name.clone());

    let hold = join_all([waker,maker]).await;

    println!("{:?}",hold);

}

async fn wakeup(channel_name:String) -> Result<String,String>{

    println!("wakeup called");

    let hold = thread::spawn(move || {
        let sleep = time::Duration::from_millis(5000);
        thread::sleep(sleep);
        match CHANNELS.lock(){
            Ok(mut lock)=>{
                match lock.get_mut(&channel_name){
                    Some(waker)=>{
                        waker.wake();
                        return Ok(());
                    },
                    None=>{
                        return Err(format!("not_found-waker"));
                    }
                }
            },
            Err(_)=>{
                return Err(format!("failed-lock-channels"));
            }
        }
    });

    match hold.join(){
        Ok(_)=>{
            return Ok("waker successfull".to_string());
        },
        Err(_)=>{
            return Err("waker failed".to_string());
        }
    }

}

async fn make_channel(channel_name:String) -> Result<String,String> {

    let mut poll:Poll;
    match Poll::new(){
        Ok(o)=>{
            poll = o;
        },
        Err(e)=>{
            return Err(format!("failed start new poll : {:?}",e));
        }
    }

    let mut events = Events::with_capacity(2);
    const WAKE_TOKEN: Token = Token(10);

    match Waker::new(poll.registry(), WAKE_TOKEN){
        Ok(waker)=>{
            match CHANNELS.lock(){
                Ok(mut lock)=>{
                    lock.insert(channel_name.clone(),waker);
                },
                Err(_)=>{
                    return Err(format!("failed-lock-channels"));
                }
            }
        },
        Err(_)=>{
            return Err(format!("failed-make-waker"));
        }
    }

    match poll.poll(&mut events, None){
        Ok(_)=>{
            println!("polled");
        },
        Err(_)=>{
            return Err(format!("failed-poll-waker"));
        }
    }

    println!("polled : {:?}",channel_name);

    return Ok(channel_name);

}
