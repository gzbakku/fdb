

const LOG:bool = true;

#[allow(dead_code)]
pub fn error(e:&str) -> String {
    let format = format!("!!! {}",e);
    if LOG {
        println!("{}",&format);
    }
    return format;
}

#[allow(dead_code)]
pub fn error_format(e:String) -> Result<String,String> {
    let format = format!("!!! {}",e);
    if LOG {
        println!("{}",&format);
    }
    return Err(format);
}
