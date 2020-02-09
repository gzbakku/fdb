use actix_web::{web, App, HttpServer};

mod router;
use router::{vault,control,files,list};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod auth;
use auth::Auth;

pub fn init(port:String,cert_path:String,key_path:String) -> std::io::Result<()> {

    let addr = String::from(format!("127.0.0.1:{}",port));
    println!("@@@ listening on port {}",&addr);

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file(key_path, SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(cert_path).unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(Auth)
            .data(web::JsonConfig::default().limit(40096))
            .service(
                web::resource("/write/encrypted").route(web::post().to_async(vault::write))
            )
            .service(
                web::resource("/read/encrypted").route(web::post().to_async(vault::read))
            )
            .service(
                web::resource("/write/json").route(web::post().to_async(files::write))
            )
            .service(
                web::resource("/read/json").route(web::post().to_async(files::read))
            )
            .service(
                web::resource("/list").route(web::post().to_async(list::list))
            )
            .service(
                web::resource("/kill").route(web::post().to_async(control::kill))
            )
            .service(
                web::resource("/check").route(web::post().to_async(control::check))
            )
            .service(
                web::resource("/live").route(web::get().to_async(control::live))
            )
    })
    .bind_ssl(addr, builder)?
    //.bind(addr)?
    .run()

}
