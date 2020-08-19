use postoffice::server::{Request,Response,init,auth};
use postoffice::resp;

fn main() {

    let key = "0554ac53f239c96279a3cff5cb29b085".to_string();
    let address = String::from("127.0.0.1:5201");
    init(address,key,handler,auth);                             //init server here

    println!("Hello, world!");
    
}

fn auth(_:auth::Token) -> bool {
    //println!("token : {:?}",token);
    return true;
}

fn handler(req: Request) -> Result<Response,String> {

    return Ok(resp::error(req,"no_error".to_string()));

}
