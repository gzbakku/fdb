mod server;

use server::{Request,Response};

fn handler(req: Request) -> Result<Response,String> {

    let message = format!("hello client this is the server");

    //println!("request : {:?}",&req);

    match Response::new(req,message,false) {
        Ok(res)=>{
            return Ok(res);
        },
        Err(_)=>{
            println!("failed to send response");
            return Err("failed to build response".to_string());
        }
    }

}

fn auth(token:server::auth::Token) -> bool {
    //println!("token : {:?}",token);
    return true;
}

fn main(){

    let key = "8cfb30b34977529853bbe46afdbbd5ae".to_string();

    let address = String::from("127.0.0.1:5200");

    server::init(address,key,handler,auth);

}
