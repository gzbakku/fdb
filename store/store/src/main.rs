use postoffice::{server,resp,common};
use json::JsonValue;

use postoffice::check::{Field,Format};

fn main() {
    let key = "8cfb30b34977529853bbe46afdbbd5ae".to_string();
    let address = String::from("127.0.0.1:5200");
    server::init(address,key,handler,auth);
}

fn auth(_token:server::auth::Token) -> bool {
    true
}

fn handler(req: server::Request) -> Result<server::Response,String> {

    let mut new_format = Format::new();
    new_format.field_builder(vec![
        Field::new("string",false,"type",vec!["write","read","collection_check","collection_insert"],Field::no_format(),0,0,false),
        Field::new("object",false,"data",Field::no_options(),Field::no_format(),0,0,false)
    ]);

    let body:JsonValue;
    match check::check_request(req.clone(),new_format) {
        Ok(parsed)=>{
            body = parsed;
        },
        Err(e)=>{
            let error = format!("check request failed error : {:?}",e);
            return Ok(resp::error(req,error));
        }
    }

    let child_format = Format::builder(vec![
        Field::new("string",false,"id",Field::no_options(),Field::no_format(),0,0,false),
        Field::new("string",false,"path",Field::no_options(),Field::no_format(),0,0,false),
        Field::new("array",false,"users",Field::no_options(),Field::no_format(),0,100,true),
    ]);

    match check::check_children(&body["data"], "object".to_string(), Field::no_options(), child_format, false, true) {
        Ok(_)=>{},
        Err(e)=>{
            let error = format!("check children failed error : {:?}",e);
            return Ok(resp::error(req,error));
        }
    }

    let mut user_format = Format::new();
    user_format.field_builder(vec![
        Field::new("string",false,"name",Field::no_options(),Field::no_format(),0,0,false),
        Field::new("string",false,"email",Field::no_options(),Field::no_format(),0,0,false),
        Field::new("string",false,"company",Field::no_options(),Field::no_format(),2,50,true),
    ]);

    for entry in body["data"].entries() {

        let data = entry.1;
        let users = &data["users"];

        match check::check_array(&users, "object".to_string(), Field::no_options(), &user_format) {
            Ok(_)=>{},
            Err(e)=>{
                let error = format!("check array failed error : {:?}",e);
                return Ok(resp::error(req,error));
            }
        }

    }

    return Ok(resp::ok(req));

}
