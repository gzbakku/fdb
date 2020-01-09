use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn hash(base:String) -> String {
    format!("{:?}",md5::compute(base.as_bytes()))
}

pub fn uid() -> String {
    thread_rng()
    .sample_iter(&Alphanumeric)
    .take(32)
    .collect()
}

pub fn log(m:&str){
    println!(">>> {}",m);
}

pub fn log_string(m:String){
    println!(">>> {}",m);
}

pub fn error(e:&str) -> String {
    println!("!!! {}",e);
    e.to_string()
}

pub fn question(e:&str){
    println!("");
    println!("??? ..............................");
    println!("");
    println!("{}",e);
    println!("");
    //println!("__________________________________");
    //println!("");
}

pub fn answer(){
    println!(" ");
    println!(".............................. ???");
    println!("");
}

pub fn space(){
    println!("-----------------------------------");
}

pub fn line(){
    println!("");
}

pub mod time {

    use std::time::SystemTime;

    pub fn now() -> Result<String,String> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(v)=>{
                //println!("time since unix epoch {:?}",v);
                Ok(v.as_millis().to_string())
            },
            Err(e)=>{
                println!("error in fetch time since unix epoch {:?}",e);
                Err(e.to_string())
            }
        }
    }

}

pub mod request {

    pub mod post {

        use reqwest::header::{HeaderMap,HeaderValue,HeaderName};
        use reqwest;
        use json;
        use std::collections::HashMap;

        #[derive(Debug)]
        pub struct RESPONSE {
            pub success:bool,
            pub found_body:bool,
            pub body:json::JsonValue,
            pub error:String
        }

        pub fn object(url:String,obj:json::JsonValue,headers:HashMap<&str,String>) -> Result<RESPONSE,String>{

            // let client = reqwest::Client::new();
            // let res = client.get("https://www.rust-lang.org")
            // .header(USER_AGENT, "foo")
            // .send()?;

            let object_as_string:String = obj.pretty(0);

            let header_map:HeaderMap;
            match make_headers(headers) {
                Ok(map)=>{header_map = map;},
                Err(_)=>{
                    return Err(crate::common::error("failed-parse_headermap"));
                }
            }

            let url_string_to_object = reqwest::Url::parse(&url).unwrap();

            let client = reqwest::Client::new();
            let send = client.post(url_string_to_object)
                .headers(header_map)
                .body(object_as_string)
                .send();

            match send {
                Ok(mut res) => {

                    match res.text() {
                        Ok(r)=>{
                            match json::parse(&r) {
                                Ok(parsed)=>{
                                    match check_request_object(parsed) {
                                        Ok(r)=>{
                                            return Ok(r);
                                        },
                                        Err(_)=>{
                                            return Err(crate::common::error("failed-check_request_object"));
                                        }
                                    }
                                },
                                Err(_)=>{
                                    return Err(crate::common::error("failed-parse_response_body-to_json"));
                                }
                            }
                        },
                        Err(_)=>{
                            return Err(crate::common::error("failed-parse_response_body"));
                        }
                    }
                },
                Err(_) => {
                    return Err(crate::common::error("failed-parse_response"));
                }
            }

        }

        fn check_request_object(res:json::JsonValue) -> Result<RESPONSE,String> {

            let mut build = RESPONSE {
                success:true,
                found_body:false,
                error:String::new(),
                body:json::object!{}
            };

            if res.has_key("result") {
                match res["result"].as_str() {
                    Some(r) => {
                        if r != "success" {
                            build.success = false;
                        }
                    },
                    None=>{
                        return Err(crate::common::error("failed-fetch_result_key_boolean"));
                    }
                }
            }

            if res.has_key("body") == false {
                if res["body"].is_object() {
                    build.found_body = false;
                    build.body = res["body"].clone();
                }
            }

            return Ok(build);

        }

        fn make_headers(headers:HashMap<&str,String>) -> Result<HeaderMap,String> {

            let mut map = HeaderMap::new();

            let mut build_failed = false;
            for key in headers.keys(){
                match headers.get(key) {
                    Some(v)=>{
                        let value_as_headermap_object:HeaderValue = v.parse().unwrap();
                        let key_as_headername = HeaderName::from_lowercase(key.as_bytes()).unwrap();
                        map.insert(key_as_headername,value_as_headermap_object);
                    },
                    None=>{
                        build_failed = true;
                        break;
                    }
                }
            }

            let json_body_header:HeaderValue = "application/json".parse().unwrap();
            map.insert(reqwest::header::CONTENT_TYPE,json_body_header);

            if build_failed {
                return Err(crate::common::error("failed-fetch_headedr_value_from_hashmap"));
            } else {
                return Ok(map);
            }

        }

    }

}
