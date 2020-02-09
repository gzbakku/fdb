

const log:bool = true;

pub fn error(e:&str) -> String {
    let format = format!("!!! {}",e);
    if log {
        println!("{}",&format);
    }
    return format;
}

pub fn error_format(e:String) -> Result<String,String> {
    let format = format!("!!! {}",e);
    if log {
        println!("{}",&format);
    }
    return Err(format);
}
