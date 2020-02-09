mod server;

use server::{Request,Response};

fn handler(req: Request) -> Result<Response,String> {

    let message = format!("hello client this is the server {}",req.data);

    match Response::new(req,message) {
        Ok(res)=>{
            return Ok(res);
        },
        Err(_)=>{
            println!("failed to send response");
            return Err("failed to build response".to_string());
        }
    }

}

fn main(){

    let key = "8cfb30b34977529853bbe46afdbbd5ae".to_string();

    let address = String::from("127.0.0.1:5200");

    server::init(address,key,handler);

}
