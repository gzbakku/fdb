mod collector;
mod io;
use std::{thread,time};

fn main(){

    let this_dir = "d://workstation/expo/rust/fdb";
    let base_dir = format!("{}/vault/",this_dir);
    io::ensure_dir(&base_dir);

    // println!("{:?}",&base_dir);
    match collector::init(base_dir){
        Ok(_)=>{},
        Err(e)=>{
            println!("{:?}",e);
            println!("failed init controller");
            return;
        }
    }

    let start_from = 3;

    for i in 0..start_from{
        // println!("{:?}",i);
        collector::insert(&i.to_string()).unwrap();
    }

    writer_thread(start_from);
    close_collector();
    process_collections();

}

fn close_collector(){
    thread::spawn(move || {
        let sleep = time::Duration::from_millis(15000);
        thread::sleep(sleep);
        match collector::close(){
            Ok(_)=>{
                println!("closing closer loop");
                return;
            },
            Err(e)=>{
                println!("failed close collector : {:?}",e);
            }
        }
    });
}

fn writer_thread(start_from:i32){
    thread::spawn(move || {
        let mut loop_no = start_from;
        loop_no += 1;
        loop {
            for _ in 0..start_from{
                match collector::insert(&loop_no.to_string()){
                    Ok(_)=>{
                        loop_no += 1;
                    },
                    Err(_e)=>{
                        println!("closing writer loop : {:?}",loop_no);
                        return;
                    }
                }
            }
            let sleep = time::Duration::from_millis(1000);
            thread::sleep(sleep);
        }
    });
}

fn process_collections(){
    loop {
        match loop_collections(){
            Ok(_)=>{},
            Err(e)=>{
                if e == "no_collections_found"{
                    // println!("closing collections loop");
                    // return;
                }
                let sleep = time::Duration::from_millis(3000);
                thread::sleep(sleep);
            }
        }
    }
}

fn loop_collections() -> Result<(),&'static str>{
    match collector::collection::get(true){
        Ok(mut collection)=>{
            if true{
                println!("{:?}",collection.data);
                match collection.flush(){
                    Ok(_)=>{
                        println!("successfull flush collection");
                        return Ok(());
                    },
                    Err(_)=>{
                        println!("failed flush collection");
                    }
                }
            } else {
                println!("{:?}",collection.data);
            }
        },
        Err(e)=>{
            if e == "no_collections_found"{
                // println!("{:?}",e);
                return Err("no_collections_found");
            }
            println!("failed get collection : {:?}",e)
        }
    }
    return Err("no_error");
}
