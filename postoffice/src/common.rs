use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[allow(dead_code)]
pub fn log(m:&str){
    println!(">>> {}",m);
}

#[allow(dead_code)]
pub fn error(e:&str) -> String {
    let error = format!("!!! {}",e);
    println!("{}",&error);
    return error.to_string();
}

pub fn uid(len:usize) -> String {
    let gen: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .collect();
    return gen;
}

pub mod hash {
    use sha2::{Sha256, Digest};
    use base64::encode;
    use md5 as md5_engine;
    pub fn md5(m:&String) -> String {
        format!("{:?}",md5_engine::compute(m.as_bytes()))
    }
    pub fn sha256(m:&String) -> String {
        let result = Sha256::digest(m.as_bytes());
        return encode(&result);
    }
}
