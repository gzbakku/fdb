use actix_web::{web, App, HttpServer};

mod router;
use router::{control,files,list};

mod auth;
use auth::Auth;

pub fn init(port:String,cert_path:String,key_path:String) -> std::io::Result<()> {

    let addr = String::from(format!("127.0.0.1:{}",port));
    println!("@@@ listening on port {}",&addr);

    HttpServer::new(|| {
        App::new()
            .wrap(Auth)
            .data(web::JsonConfig::default().limit(40096))
            .service(
                web::resource("/write").route(web::post().to_async(files::write))
            )
            .service(
                web::resource("/read").route(web::post().to_async(files::read))
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
    .bind(addr)?
    .run()

}
