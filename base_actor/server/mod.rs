mod router;
mod auth;
use router::{vault,files,control,list};

use actix_web::{web, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_rt::main]
pub async fn init(port:String,cert:String,key:String) -> std::io::Result<()> {

    let addr = String::from(format!("127.0.0.1:{}",port));
    println!("@@@ listening on port {}",&addr);

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file(key, SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(cert).unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(auth::Auth)
            .data(web::JsonConfig::default().limit(40096))
            .service(
                web::resource("/write/encrypted").route(web::post().to(vault::write))
            )
            .service(
                web::resource("/read/encrypted").route(web::post().to(vault::read))
            )
            .service(
                web::resource("/write/json").route(web::post().to(files::write))
            )
            .service(
                web::resource("/read/json").route(web::post().to(files::read))
            )
            .service(
                web::resource("/list").route(web::post().to(list::list))
            )
            .service(
                web::resource("/kill").route(web::post().to(control::kill))
            )
            .service(
                web::resource("/check").route(web::post().to(control::check))
            )
            .service(
                web::resource("/").route(web::get().to(control::live))
            )
    })
    .bind_openssl(addr, builder)?
    //.bind(addr)?
    .run()
    .await

}
