use postoffice::resp;
use postoffice::server::{Request,Response};
use json::JsonValue;

pub fn init(req:Request,body:&JsonValue) -> Response {
    let object = JsonValue::new_object();
    return resp::data(req,object,false);
}
