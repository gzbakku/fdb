use actix_web::{
    http, error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use bytes::{Bytes, BytesMut};
use json::JsonValue;

use futures::future::{ok, Either, Ready};
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use std::task::{Context, Poll};

mod server;

//********************************
//auth

pub struct Auth;

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckRequest<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckRequest { service })
    }
}
pub struct CheckRequest<S> {
    service: S,
}

impl<S, B> Service for CheckRequest<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {

        let connection_info = req.connection_info().clone();
        let peer = connection_info.remote();

        println!("peer : {:?}",peer);

        let verified = true;

        if verified {
            Either::Left(self.service.call(req))
        } else {
            Either::Right(ok(req.into_response(
                HttpResponse::Forbidden()
                .set_header("forbidden", "true")
                .finish()
                .into_body()
            )))
        }
    }
}

pub async fn test_json(body: Bytes) -> Result<HttpResponse, Error> {

    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    let injson: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string() },
    };

    return Ok(server::success());

}

pub async fn live(_: Bytes) -> Result<HttpResponse, Error> {

    return Ok(server::success());

}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let cert = "d://workstation/expo/rust/fdb/cert/keys/cert.pem";
    let key = "d://workstation/expo/rust/fdb/cert/keys/key.pem";

    let port = 7080;
    let address = format!("127.0.0.1:{}",port);
    println!("@@@ server live at https://{}",&address);

    // load ssl keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(key, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(cert).unwrap();

    HttpServer::new(||
        App::new()
        .wrap(Auth)
        .route("/", web::get().to(live))
        .route("/test", web::post().to(test_json))
    )
        .bind_openssl(address, builder)?
        .run()
        .await
}
