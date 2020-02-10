use postoffice::server;
use json::JsonValue;
mod resp;

fn main() {
    let key = "8cfb30b34977529853bbe46afdbbd5ae".to_string();
    let address = String::from("127.0.0.1:5200");
    server::init(address,key,handler,auth);
}

fn auth(tokne:server::auth::Token) -> bool {
    true
}

fn handler(req: server::Request) -> Result<server::Response,String> {

    let message = format!("hello client this is the server");

    let body:JsonValue;
    match resp::get_body(&req) {
        Ok(parsed)=>{
            body = parsed;
        },
        Err(_)=>{
            return Ok(resp::error(req,"failed to parse body".to_string()));
        }
    }

    return Ok(resp::ok(req));

    // match server::Response::new(req,message) {
    //     Ok(res)=>{
    //         return Ok(res);
    //     },
    //     Err(_)=>{
    //         println!("failed to send response");
    //         return Err("failed to build response".to_string());
    //     }
    // }

}
