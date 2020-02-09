use actix_web::{Error, HttpResponse};
use crate::{common,resp,SESSION};
use std::thread;
use std::time::Duration;
use bytes::Bytes;
use json::object;

pub async fn live(_: Bytes) -> Result<HttpResponse, Error> {
    return Ok(resp::success());
}

pub async fn check(_: Bytes) -> Result<HttpResponse, Error> {

    let session_as_mutex = SESSION.lock().unwrap();
    let session_as_template = session_as_mutex.copy();
    let session_id = session_as_template.id;
    let session_as_json = object!{
        "session_id" => session_id
    };
    return Ok(resp::success_with_data(session_as_json));

}

pub async fn kill(_body: Bytes) -> Result<HttpResponse, Error> {
    end();
    return Ok(resp::success());
}

pub fn end(){
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(3000));
        common::log("killing files actor");
        std::process::exit(1);
    });
}
