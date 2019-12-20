
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn hash(base:String) -> String {
    format!("{:?}",md5::compute(base.as_bytes()))
}

pub fn uid() -> String {
    thread_rng()
    .sample_iter(&Alphanumeric)
    .take(32)
    .collect()
}

pub fn log(m:&str){
    println!(">>> {}",m);
}

pub fn log_string(m:String){
    println!(">>> {}",m);
}

pub fn error(e:&str){
    println!("!!! {}",e);
}

pub fn question(e:&str){
    println!("");
    println!("??? ..............................");
    println!("");
    println!("{}",e);
    println!("");
    //println!("__________________________________");
    //println!("");
}

pub fn answer(){
    println!(" ");
    println!(".............................. ???");
    println!("");
}

pub fn space(){
    println!("-----------------------------------");
}

pub fn line(){
    println!("");
}

pub mod time {

    use std::time::SystemTime;

    pub fn now() -> Result<String,String> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(v)=>{
                //println!("time since unix epoch {:?}",v);
                Ok(v.as_millis().to_string())
            },
            Err(e)=>{
                println!("error in fetch time since unix epoch {:?}",e);
                Err(e.to_string())
            }
        }
    }

}
