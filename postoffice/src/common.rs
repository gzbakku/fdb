
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
