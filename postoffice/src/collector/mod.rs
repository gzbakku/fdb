use lazy_static::lazy_static;
use std::sync::Mutex;
use std::fs::{File};
use std::io::Write;
use std::{thread,time};

mod looper;
pub mod io;
pub mod control;
pub mod collection;

use control::Control;

#[derive(Debug)]
struct Collector {
    writer:File,
    dir:String,
    name:String,
    empty:bool
}

#[allow(dead_code)]
impl Collector {
    #[allow(dead_code)]
    fn new() -> Collector {
        Collector {
            writer:File::create("hundred").expect("failedt ocreate base file for collector"),
            dir:String::new(),
            name:String::new(),
            empty:true
        }
    }
    #[allow(dead_code)]
    pub fn overtake(self:&mut Self,base_dir:&String,collection:&String) -> Result<(),String> {
        let collection_path = format!("{}/{}.fdbcs",&base_dir,&collection);
        if !io::check_path(&collection_path) {
            return Err("failed-check_collection_path".to_string());
        }
        match File::create(&collection_path) {
            Ok(writer)=>{
                self.writer = writer;
                self.dir = base_dir.clone();
                self.name = collection.clone();
                self.empty = true;
                return Ok(());
            },
            Err(e)=>{
                let error = format!("failed-open_collection_file=>{}",e);
                return Err(error);
            }
        }
    }
}

#[derive(Debug,Clone)]
pub struct BaseDir {
    pub path:String
}

#[allow(dead_code)]
impl BaseDir {
    #[allow(dead_code)]
    pub fn new() -> BaseDir {
        BaseDir {
            path:String::new()
        }
    }
    #[allow(dead_code)]
    pub fn overtake(self:&mut Self,path:&String){
        self.path = path.to_string();
    }
    #[allow(dead_code)]
    pub fn path(self:Self) -> String {
        return self.path;
    }
}

#[derive(Debug,Clone)]
pub struct Collections {
    pub reset:Vec<String>,
    pub flush:Vec<String>
}

#[derive(Debug,Clone,Copy)]
pub struct Close {
    pub switch:bool,
    pub safe:bool
}

impl Close{
    fn new()->Close{
        Close{
            switch:false,
            safe:false
        }
    }
    fn should_close(self)->bool{
        return self.switch;
    }
    fn is_it_safe(self)->bool{
        return self.safe;
    }
}

lazy_static! {
    static ref COLLECTIONS: Mutex<Collections> = Mutex::new(Collections {
        reset:Vec::new(),
        flush:Vec::new()
    });
    static ref BASE_DIR: Mutex<BaseDir> = Mutex::new(BaseDir::new());
    static ref CONTROL: Mutex<Control> = Mutex::new(Control::new());
    static ref ACTIVE: Mutex<Collector> = Mutex::new(Collector::new());
    static ref CLOSE: Mutex<Close> = Mutex::new(Close::new());
}

#[allow(dead_code)]
pub fn insert(data:&String) -> Result<(),String> {

    match CLOSE.lock() {
        Ok(closer)=>{
            if closer.should_close() {
                return Err("collector is closed".to_string());
            }
        },
        Err(_)=>{
            println!("closer fetch error");
            return Err("failed-lock_Closer-close_collector".to_string());
        }
    }

    let in_line = format!("{}\n",data);
    match ACTIVE.lock() {
        Ok(mut collector)=>{
            match collector.writer.write(in_line.as_bytes()) {
                Ok(_)=>{
                    collector.empty = false;
                    return Ok(());
                },
                Err(e)=>{
                    let error = format!("failed-write_to_active_collection=>{}",e);
                    return Err(error);
                }
            }
        },
        Err(_)=>{
            return Err("failed-lock_active-overtake_new_collector".to_string());
        }
    }
}

#[allow(dead_code)]
pub fn init(path:String) -> Result<(),String> {

    if !io::check_path(&path) {
        return Err(String::from("dir does not exists"));
    }

    match BASE_DIR.lock(){
        Ok(mut base_dir)=>{
            base_dir.overtake(&path);
        },
        Err(_)=>{
            return Err(String::from("failed-lock_base_dir_mutex"));
        }
    }

    let active:String;
    match control::init(&path) {
        Ok(crl)=>{
            active = crl.active.clone();
            match CONTROL.lock() {
                Ok(mut hold)=>{
                    hold.overtake(crl);
                },
                Err(_)=>{
                    return Err("failed-lock_control-overtake_new_control_struct".to_string());
                }
            }
        },
        Err(e)=>{
            println!("failed-fetch_control error : {:?}",e);
            return Err("failed-fetch_control".to_string());
        }
    }

    match ACTIVE.lock() {
        Ok(mut collector)=>{
            match collector.overtake(&path,&active) {
                Ok(_)=>{},
                Err(e)=>{
                    let error = format!("failed-overtake_new_collection=>{}",e);
                    return Err(error);
                }
            }
        },
        Err(_)=>{
            return Err("failed-lock_active-overtake_new_collector".to_string());
        }
    }

    looper::reset_loop(&path);

    return Ok(());

}

#[allow(dead_code)]
pub fn close() -> Result<(),&'static str>{

    match CLOSE.lock() {
        Ok(mut closer)=>{
            closer.switch = true;
        },
        Err(_)=>{
            return Err("failed-lock_Closer-close_collector");
        }
    }

    let handle:std::thread::JoinHandle<std::result::Result<(), &'static str>> = thread::spawn(move || {
        loop {
            match CLOSE.lock() {
                Ok(closer)=>{
                    if closer.is_it_safe() {
                        return Ok(());
                    }
                },
                Err(_)=>{
                    println!("failed-lock_Closer-close_collector");
                }
            }
            let sleep = time::Duration::from_millis(3000);
            thread::sleep(sleep);
        }
    });
    match handle.join() {
        Ok(_)=>{
            return Ok(());
        },
        Err(_)=>{
            return Err("failed-close-collector");
        }
    }

}
