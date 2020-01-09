use actix_web::{web, error, Error, App, HttpRequest, HttpServer, HttpResponse, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use futures::{StreamExt};
use bytes::{Bytes};
use json::JsonValue;

mod server;

async fn index(body: Bytes) -> Result<HttpResponse, Error> {

    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    let mut injson: JsonValue = json::object! {"err" => "parse-failed" };
    match result {
        Ok(r) => {
            injson = r;
        },
        Err(e) => {
            return Ok(server::error("parse-failed"));
        }
    };

    println!("Body {:?}!", injson);

    return Ok(server::success());

}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let cert = "D://workstation/expo/rust/fdb/files/keys/cert.pem".to_string();
    let key = "D://workstation/expo/rust/fdb/files/keys/key.pem".to_string();

    // load ssl keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(key, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(cert).unwrap();

    HttpServer::new(|| {
        App::new()
        .service(web::resource("/").route(web::post().to(index)))
        //.route("/", web::post().to(index))
    })
        .bind_openssl("127.0.0.1:7560", builder)?
        //.bind("127.0.0.1:7560")?
        .run()
        .await

}
