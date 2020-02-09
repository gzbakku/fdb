use futures::future::{ok, Either, Ready};
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use std::task::{Context, Poll};

use actix_web::{Error,HttpResponse};

pub struct Auth;

mod auth;

use crate::{BASE_KEY,COMPOSER,NODE,common};

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
        let headers = req.headers();
        let path = req.path();
        let path_as_string = String::from(path);
        let base_key = BASE_KEY.lock().unwrap()[0].to_string();
        let node_as_mutex = NODE.lock().unwrap();
        let node_as_template = node_as_mutex.copy();
        let composer_as_mutex = COMPOSER.lock().unwrap();
        let composer_as_template = composer_as_mutex.copy();

        let peer = connection_info.remote();

        let mut access_granted = true;

        let mut peer_as_string:String = String::new();
        match auth::extract_peer(peer) {
            Ok(r)=>{
                peer_as_string = r.to_string();
            },
            Err(_)=>{
                common::error("failed-extract-peer-authenticate-files");
                access_granted = false;
            }
        }

        if access_granted {
            match auth::check(headers,base_key,node_as_template,peer_as_string,path_as_string,composer_as_template) {
                Ok(_)=>{},
                Err(_) => {
                    access_granted = false;
                }
            }
        }

        if access_granted {
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
