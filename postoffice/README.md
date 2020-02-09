# Post Office

this is a light weight tcp connection manager, including client and server side functions to communicate at high speed long living stable connctions with message que written in Rust.

## Installation

put carte name and version in your cargo.toml file under dependencies.

```bash
postoffice = "*"
```

## Server Usage

```rust

use postoffice::{Request,Response,server};

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

```

## Client Usage

```rust

use postoffice::client::{get_test_message,get_random_connection_id,start_connection,send_message};

let key = "8cfb30b34977529853bbe46afdbbd5ae".to_string();
let connection_id = client::get_random_connection_id();
let addr = "127.0.0.1:5200".to_string();

match start_connection(&connection_id,addr,key) {
   Ok(_)=>{
       //println!("connection establishged");
   },
   Err(_)=>{
       common::error("failed start connection");
   }
}

let message = get_test_message(8);
match send_message(&connection_id, message.clone(), false) {
   Ok(response)=>{
    if response.message.contains(&message) {
     if false {
       println!("request successfull");
     }
     if false {
       println!("response final : {:#?}",response);
     }
     } else {
       println!("response final : {:?}",response);
     }
    },
   Err(_)=>{
     common::error("request-failed");
   }
}

```


Please make sure to update tests as appropriate.

## License
MIT
