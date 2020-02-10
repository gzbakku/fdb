use json::{JsonValue,parse};
use postoffice::server::{Response,Request};

pub fn get_body(req:&Request) -> Result<JsonValue,String> {
    match parse(&req.data) {
        Ok(parsed)=>{
            return Ok(parsed);
        },
        Err(_)=>{
            return Err("failed to parse request data".to_string());
        }
    }
}

pub fn error(req:Request,error:String) -> Response {
    let mut object = JsonValue::new_object();
    match object.insert("result",false) {
        Ok(_)=>{},
        Err(_)=>{}
    }
    match object.insert("error",error) {
        Ok(_)=>{},
        Err(_)=>{}
    }
    return new_response(req,object);
}

pub fn ok(req:Request,) -> Response {
    let mut object = JsonValue::new_object();
    match object.insert("result",true) {
        Ok(_)=>{},
        Err(_)=>{}
    }
    return new_response(req,object);
}

pub fn data(req:Request,data:JsonValue) -> Response {
    let mut object = JsonValue::new_object();
    match object.insert("result",true) {
        Ok(_)=>{},
        Err(_)=>{}
    }
    match object.insert("data",data) {
        Ok(_)=>{},
        Err(_)=>{}
    }
    return new_response(req,object);
}

pub fn new_response(req:Request,data:JsonValue) -> Response {
    let data_as_string = data.dump();
    match Response::new(req,data_as_string) {
        Ok(res)=>{
            return res;
        },
        Err(_)=>{
            return Response::error(req,"make-new_response".to_string());
        }
    }
}
