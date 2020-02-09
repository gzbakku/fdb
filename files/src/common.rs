
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[allow(dead_code)]
pub fn hash(base:String) -> String {
    format!("{:?}",md5::compute(base.as_bytes()))
}

#[allow(dead_code)]
pub fn uid() -> String {
    thread_rng()
    .sample_iter(&Alphanumeric)
    .take(32)
    .collect()
}

#[allow(dead_code)]
pub fn log(m:&str){
    println!(">>> {}",m);
}

#[allow(dead_code)]
pub fn log_string(m:String){
    println!(">>> {}",m);
}

#[allow(dead_code)]
pub fn error(e:&str) -> String {
    println!("!!! {}",e);
    e.to_string()
}

#[allow(dead_code)]
pub fn error_format(e:String) -> String {
    println!("!!! {}",e);
    e
}

#[allow(dead_code)]
pub fn question(e:&str){
    println!("");
    println!("??? ..............................");
    println!("");
    println!("{}",e);
    println!("");
    //println!("__________________________________");
    //println!("");
}

#[allow(dead_code)]
pub fn answer(){
    println!(" ");
    println!(".............................. ???");
    println!("");
}

#[allow(dead_code)]
pub fn space(){
    println!("-----------------------------------");
}

#[allow(dead_code)]
pub fn line(){
    println!("");
}

#[allow(dead_code)]
pub fn success(){
    println!("@@@");
}

pub mod time {

    use std::time::SystemTime;

    #[allow(dead_code)]
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
