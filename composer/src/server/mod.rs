#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use std::collections::HashMap;

use actix_web::{
    web, App, Error, HttpResponse, Httpresponses
};
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceResponse,ServiceRequest};

use futures::{Future, Stream, Poll};
use futures::future::{ok, Either, FutureResult};
use json::JsonValue;

use responses;

lazy_static! {

    #[derive(Debug)]
    static ref KEY: Mutex<Vec<String>> =  Mutex::new(vec![]);
    static ref TYPE: Mutex<Vec<String>> =  Mutex::new(vec![]);
    static ref ACTORS : Mutex<HashMap<String,HashMap<String,String>>> = Mutex::new(HashMap::new());
    //static ref CONFIG : Mutex<JsonValue> = Mutex::new(JsonValue::new_object());
}

//*******************************************************
//authenticate

pub struct Auth;

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckIp<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckIp { service })
    }
}

pub struct CheckIp<S> {
    service: S,
}

impl<S, B> Service for CheckIp<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {

        let connection_info = req.connection_info().clone();

        //let peer = connection_info.remote();
        //println!("peer : {:?}", peer);

        let access_granted = true;

        if access_granted {
            Either::A(self.service.call(req))
        } else {
            Either::B(ok(req.into_response(
                HttpResponse::Forbidden()
                .set_header("forbidden", "true")
                .finish()
                .into_body()
            )))
        }

    }
}

//*******************************************************
//start the responses

fn main(port:String,base_dir:String){

    //check the ssl files

    //generate the ssl files if necessary

    //start the responses

    server();

}

//*******************************************************
//responses functions

fn server(port:String,private_path:String,public_path:String) -> std::io::Result<()> {

    let addr = String::from(format!("127.0.0.1:{}",port));
    println!("composer responses listening on port {}",&addr);

    /*
        services :-
            get actors
            check if live
            type check
    */

    Httpresponses::new(|| {
        App::new()
            .wrap(Auth)
            .data(web::JsonConfig::default().limit(40096))
            .service(
                web::resource("/live").route(web::post().to_async(write_encrypted))
            )
            .service(
                web::resource("/fetch_actors").route(web::post().to_async(write_encrypted))
            )
            // .service(
            //     web::resource("/fetch_actors").route(web::post().to_async(write_encrypted))
            // )
    })
    .bind(addr)?
    .run()

}

fn live(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|body| {

        let result = json::parse(std::str::from_utf8(&body).unwrap());
        let injson: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };

        if &injson.has_key("file") == &false {
            return Ok(responses::error("not_found-file_name".to_string()));
        }
        if &injson.has_key("marker") == &false {
            return Ok(responses::error("not_found-file_data".to_string()));
        }
        if &injson["data"].is_object() == &false {
            return Ok(responses::error("invalid-file_data".to_string()));
        }

        let opener = &KEY.lock().unwrap();
        let key = &opener[0];

        let data_dump = &injson["data"].dump();
        let encrypted = crypt::encrypt(data_dump.to_string(),key.to_string());
        let combine = format!("{:?};{:?}",encrypted.nonce,encrypted.cipher);
        let get_dir = DIR.lock().unwrap()[0].to_string();

        match io::crypted::write(get_dir,injson["file"].to_string(),combine.as_bytes().to_vec()) {
            Ok(_r) => {},
            Err(e) => {
                return Ok(responses::error(e.to_string()));
            }
        }

        let open_book = &mut BOOK.lock().unwrap();
        open_book.insert(injson["file"].to_string(),injson["data"].clone());

        return Ok(responses::success());

    })

}
