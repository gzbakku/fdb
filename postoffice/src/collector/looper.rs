use crate::collector::{ACTIVE,CONTROL,control,COLLECTIONS,Collections,BASE_DIR,CLOSE};
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
pub fn reset_loop(base_dir_ref:&String){
    let base_dir = base_dir_ref.clone();
    thread::spawn(move|| {
        loop {
            match process_collection_edits() {
                Ok(_)=>{},
                Err(e)=>{
                    println!("!!! failed-process_collection_edits=>{}",e);
                }
            }
            if check_if_writen() == false || false {
                reset(&base_dir);
            }
            match CLOSE.lock() {
                Ok(mut closer)=>{
                    if closer.should_close(){
                        thread::sleep(Duration::from_millis(3000));
                        closer.safe = true;
                        return;
                    }
                },
                Err(_)=>{
                    println!("failed-lock_Closer-close_collector");
                }
            }
            thread::sleep(Duration::from_millis(5000));
        }
    });
}

fn process_collection_edits() -> Result<(),String> {

    let base_dir:String;
    match BASE_DIR.lock(){
        Ok(bd)=>{
            base_dir = bd.path.clone();
        },
        Err(_)=>{
            return Err("failed-lock_base_dir_mutex".to_string());
        }
    }

    let edits:Collections;
    match COLLECTIONS.lock() {
        Ok(e)=>{
            if e.flush.len() == 0 {
                return Ok(());
            }
            edits = e.clone();
        },
        Err(_)=>{
            return Err("failed-lock_COLLECTIONS_mutex".to_string());
        }
    }

    println!("--- do not close commiting collection edits ---");

    let mut base_collections:Vec<String>;
    let active:String;
    match CONTROL.lock() {
        Ok(e)=>{
            active = e.active.clone();
            base_collections = e.finished.clone();
        },
        Err(_)=>{
            return Err("failed-lock_CONTROL_mutex".to_string());
        }
    }

    let mut index = 0;
    for coll in base_collections.clone() {
        if edits.flush.contains(&coll) {
            base_collections.remove(index);
        }
        index += 1;
    }

    let mut collection_string = String::new();
    for coll in base_collections {
        collection_string.push_str(&coll);
        collection_string.push_str(",");
    }

    let control_path = format!("{}/control.fdbc",&base_dir);
    let control_string = format!("{};{}",active,collection_string);

    match control::edit_control(control_path,control_string) {
        Ok(_)=>{},
        Err(e)=>{
            let error = format!("failed-edit_control=>{}",e);
            return Err(error);
        }
    }

    match COLLECTIONS.lock() {
        Ok(mut e)=>{
            e.reset = Vec::new();
            e.flush = Vec::new();
        },
        Err(_)=>{
            return Err("failed-lock_COLLECTIONS_mutex".to_string());
        }
    }

    println!("--- safe to close now ---");

    return Ok(());

}

#[allow(dead_code)]
fn check_if_writen() -> bool {
    match ACTIVE.lock() {
        Ok(collector)=>{
            return collector.empty;
        },
        Err(_)=>{
            return true;
        }
    }
}

#[allow(dead_code)]
fn reset(base_dir:&String){
    println!("--- do not close reseting active collection ---");
    match CONTROL.lock() {
        Ok(mut cont)=>{
            let control_path = format!("{}/control.fdbc",base_dir);
            let mut finished = cont.finished.clone();
            finished.push(cont.active.clone());
            match control::update_control(&base_dir, &control_path, finished) {
                Ok(crl)=>{
                    cont.overtake(crl.clone());
                    match ACTIVE.lock() {
                        Ok(mut collector)=>{
                            match collector.overtake(&base_dir,&crl.active.clone()) {
                                Ok(_)=>{},
                                Err(_)=>{}
                            }
                        },
                        Err(_)=>{}
                    }
                },
                Err(_)=>{}
            }
        },
        Err(_)=>{}
    }
    println!("--- safe to close now ---");
}
