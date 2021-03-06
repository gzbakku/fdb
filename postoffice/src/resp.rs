use json::{JsonValue,parse};
use crate::{server,client};

#[derive(Debug,Clone)]
pub struct Resp {
    pub result:bool,
    pub error:String,
    pub data:JsonValue,
    pub is_there_error:bool,
    pub is_there_data:bool
}

#[allow(dead_code)]
impl Resp {
    #[allow(dead_code)]
    fn check(me:&Self) -> bool {
        return me.result;
    }
    #[allow(dead_code)]
    fn error(me:&Self) -> String {
        return me.error.clone();
    }
}

#[allow(dead_code)]
pub fn parse_response(response:client::Response) -> Result<Resp,String> {

    let mut resp = Resp {
        result:false,
        error:String::new(),
        data:JsonValue::new_object(),
        is_there_data:false,
        is_there_error:false
    };

    let data;
    match parse(&response.message) {
        Ok(json)=>{
            data = json.clone();
            if &json.has_key("result") == &true {
                if
                     &json["data"].is_object() == &true ||
                     &json["data"].is_array() == &true
                {
                    resp.is_there_data = true;
                    resp.data = json["data"].clone();
                }
            }
        },
        Err(_)=>{
            return Err("failed-parse_data".to_string());
        }
    }

    if &data.has_key("result") == &false {
        return Err("failed-no_result_key".to_string());
    } else {
        match &data["result"].as_bool() {
            Some(result) => {
                if result == &true {
                    resp.result = true;
                } else if result == &false {
                    resp.result = false;
                } else {
                    return Err("failed-invalid_result_key_val".to_string());
                }
            },
            None=>{
                return Err("failed-extract_result_key".to_string());
            }
        }
    }

    if &resp.result == &false {
        if &data.has_key("error") == &true {
            match &data["error"].as_str() {
                Some(e)=>{
                    resp.is_there_error = true;
                    resp.error = e.to_string();
                },
                None=>{}
            }
        }
    }

    return Ok(resp);

}

#[allow(dead_code)]
pub fn get_body(req:&server::Request) -> Result<JsonValue,String> {
    match parse(&req.data) {
        Ok(parsed)=>{
            return Ok(parsed);
        },
        Err(_)=>{
            return Err("failed to parse request data".to_string());
        }
    }
}

#[allow(dead_code)]
pub fn error(req:server::Request,error:String) -> server::Response {
    let mut object = JsonValue::new_object();
    match object.insert("result",false) {
        Ok(_)=>{},
        Err(_)=>{}
    }
    match object.insert("error",error) {
        Ok(_)=>{},
        Err(_)=>{}
    }
    return new_response(req,object,false);
}

#[allow(dead_code)]
pub fn ok(req:server::Request) -> server::Response {
    let mut object = JsonValue::new_object();
    match object.insert("result",true) {
        Ok(_)=>{},
        Err(_)=>{}
    }
    return new_response(req,object,false);
}

#[allow(dead_code)]
pub fn data(req:server::Request,data:JsonValue,secure:bool) -> server::Response {
    let mut object = JsonValue::new_object();
    match object.insert("result",true) {
        Ok(_)=>{},
        Err(_)=>{}
    }
    match object.insert("data",data) {
        Ok(_)=>{},
        Err(_)=>{}
    }
    return new_response(req,object,secure);
}

#[allow(dead_code)]
pub fn new_response(req:server::Request,data:JsonValue,secure:bool) -> server::Response {
    let data_as_string = data.dump();
    match server::Response::new(req.clone(),data_as_string,secure) {
        Ok(res)=>{
            return res;
        },
        Err(_)=>{
            return server::Response::error(req,"make-new_response".to_string());
        }
    }
}

#[allow(dead_code)]
pub fn is_successfull(r:&JsonValue) -> bool{
    if !r.has_key("result"){return false;}
    if !r["result"].is_boolean(){return false;}
    match r["result"].as_bool(){
        Some(r)=>{
            if !r{return false;} else {return true;}
        },
        None=>{
            return false;
        }
    }
}

#[allow(dead_code)]
pub fn with_data(r:&JsonValue) -> bool{
    if !r.has_key("data"){return false;}
    if !r["data"].is_object() && !r["data"].is_array(){return false;}
    return true;
}
