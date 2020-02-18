use json::{JsonValue,parse};
use postoffice::server::Request;
use postoffice::check;
use check::{Field,Format};
use crate::path;

pub fn request(req:Request) -> Result<JsonValue,String> {
    match parse(&req.data.clone()) {
        Ok(body)=>{
            match validate_request_object(&body) {
                Ok(_)=>{
                    return Ok(body);
                },
                Err(e)=>{
                    let error = format!("failed-validate_request_object=>{}",e);
                    return Err(error);
                }
            }
        },
        Err(e)=>{
            let error = format!("failed-parse_request_as_json_object=>{}",e);
            return Err(error);
        }
    }
}

pub fn line(line:String) -> Result<JsonValue,String> {
    match parse(&line) {
        Ok(body)=>{
            match validate_request_object(&body) {
                Ok(_)=>{
                    return Ok(body);
                },
                Err(e)=>{
                    let error = format!("failed-validate_request_object=>{}",e);
                    return Err(error);
                }
            }
        },
        Err(e)=>{
            let error = format!("failed-parse_request_as_json_object=>{}",e);
            return Err(error);
        }
    }
}

fn validate_request_object(body:&JsonValue) -> Result<(),String> {

    let new_format = Format::builder(vec![
        Field::new("string",false,"type",vec!["write","read","collection_check","collection_insert"],Field::no_format(),0,0,false),
        Field::new("object",false,"data",Field::no_options(),Field::no_format(),0,0,false)
    ]);

    match check::check(&body,new_format) {
        Ok(_)=>{},
        Err(e)=>{
            let error = format!("check request failed error : {:?}",e);
            return Err(error);
        }
    }

    let child_format = Format::builder(vec![
        Field::new("string",false,"id",Field::no_options(),Field::no_format(),0,0,false),
        Field::new("string",false,"path",Field::no_options(),Field::no_format(),0,0,false),
        Field::new("object",false,"files",Field::no_options(),Field::no_format(),0,100,true)
    ]);

    match check::check_children(&body["data"], "object".to_string(), Field::no_options(), child_format, false, false) {
        Ok(_)=>{},
        Err(e)=>{
            let error = format!("check children failed error : {:?}",e);
            return Err(error);
        }
    }

    for entry in body["data"].entries() {
        let data = entry.1;
        let users = &data["files"];
        if path::check(&data["path"]) == false {
            let error = format!("check path failed, data : {:?}",&data["path"]);
            return Err(error);
        }
        match check::check_array(&users, "object".to_string(), Field::no_options(),&Field::no_format()) {
            Ok(_)=>{},
            Err(e)=>{
                let error = format!("check array failed error : {:?}",e);
                return Err(error);
            }
        }
    }

    return Ok(());

}
