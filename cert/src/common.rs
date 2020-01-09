

pub fn error(e:&str) -> String {
    let make = format!("!!! {}",e);
    println!("{}",&make);
    make
}

pub fn error_string(e:String) -> String {
    let make = format!("!!! {}",e);
    println!("{}",&make);
    make
}

pub fn log(m:&str){
    let make = format!(">>> {}",m);
    println!("{}",make);
}
