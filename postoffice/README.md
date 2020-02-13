# Post Office

this is a light weight tcp connection manager, including client and server side functions to communicate at high speed long living stable connctions with message que written in Rust.

## Installation

put carte name and version in your cargo.toml file under dependencies.

```bash
postoffice = "*"
```

# Api layers

 - TCP connection - Module : client,server
 - Json Request Result handler - Module : resp
 - Json Scheme Validation on server Side for request handling - Module : check

# Tcp Connection

base tcp connection use custom utf string as request seprated by \r\n and transferred as bytes

## client Requests

### simple request

```bash
SMPL sdf67sf678sd6f sdf78sd6f78sdfsdfsdf+=\r\n
```
here first term is the request identifier then followed by the request id and the base64 encoded string

### encrypted request

```bash
SMPL sdf67sf678sd6f sdf87sd89fsd987f789sd7f8sd97f+:sdf78sd6f78sdfsdfsdf+=\r\n
```
here first term is the request identifier then followed by the request id and the base64 encoded nonce and cipher text seprated by ':'

## Server Responses

these responses are returned by the handler function as Response Struct and parsed as given below.

### Ok Response

```rust
//encrypted
match Response::new(req,"secure message".to_string(),true) {
    Ok(res)=>{
        return Ok(res);
    },
    Err()=>{
        return Resp::error(req,"parser failed".to_string());
    }
}

//simple
match Response::new(req,"unsecure message".to_string(),false) {
    Ok(res)=>{
        return Ok(res);
    },
    Err()=>{
        return Resp::error(req,"parser failed".to_string());
    }
}
```
```bash
OK sdf67sf678sd6f sdf87sd89fsd987f789sd7f8sd97f+:sdf78sd6f78sdfsdfsdf+=\r\n
```
here first term is the result identifier then followed by the request id and the base64 encoded nonce and cipher text seprated by ':' for encrypted messages and simple base64 encoded string on normal response;

### BAD Response

bad response are what they sound like they may carry errors and request identifier but if parsing fails then bad request dont identify the request by given id.

```bash
BAD parser-failed\r\n
```
this response occurs when parse fails to process the request data.

#### custom error by hanlder

```rust
return Ok(resp::error(req,"hanlder ran out of mem".to_string()));
```
```bash
BAD sdf67sf678sd6f "hanlder ran out of mem"\r\n
```

#### undefined error by hanlder

```rust
return Ok(resp::bad(req));
```
```bash
BAD sdf67sf678sd6f undefined\r\n
```




## Server Usage

```rust

use postoffice::{server,resp,common};
use postoffice::check::{Field,Format};
use json::JsonValue;

fn handler(req: Request) -> Result<Response,String> {

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
        Field::new("array",false,"users",Field::no_options(),Field::no_format(),0,100,true)
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
        Field::new("string",false,"company",Field::no_options(),Field::no_format(),2,50,true)
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

fn auth(token:server::auth::Token) -> bool {
    println!("token : {:?}",token);
    return true;
}

fn main(){
    let key = "8cfb30b34977529853bbe46afdbbd5ae".to_string();
    let address = String::from("127.0.0.1:5200");
    server::init(address,key,handler,auth);
}

```

## Client Usage

```rust

use postoffice::client::{get_random_connection_id,start_connection,send_message};
use postoffice::resp;

fn main(){

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

  let mut user = JsonValue::new_object();
  user.insert("name","akku").unwrap();
  user.insert("email","gzbakku@gmail.com").unwrap();
  user.insert("company","daachi.in").unwrap();

  let mut users = JsonValue::new_array();
  users.push(user);

  let mut request = JsonValue::new_object();
  request.insert("collection","1a62c37cf70a74cfeb69aba742643613").unwrap();
  request.insert("path","/users/");
  request.insert("users",users);

  send(connection_id,request);

}

fn send(connection_id:String,data:json::JsonValue){

    let mut request_object = json::JsonValue::new_object();

    match request_object.insert("type","write") {
        Ok(_)=>{},
        Err(_)=>{}
    }

    match request_object.insert("data",data) {
        Ok(_)=>{},
        Err(_)=>{}
    }

    let request_string = request_object.dump();

    match client::send_message(&connection_id,request_string,false) {
        Ok(response)=>{
            match resp::parse_response(response) {
                Ok(result)=>{
                    println!("result : {:?}",result);
                },
                Err(e)=>{
                    println!("error parse response strucft : {:?}",e);
                }
            }
        },
        Err(_)=>{
            println!("request failed");
        }
    }

}

```


Please make sure to update tests as appropriate.

## License
MIT
