use actix_web::{web, Error, HttpResponse};
use futures::{Future, Stream, Poll};
use crate::{resp,common,SESSION};
use json::object;

use std::thread;
use std::time::Duration;

pub fn live(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|_body| {
        return Ok(resp::success());
    })

}

pub fn check(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|_body| {
        let session_as_mutex = SESSION.lock().unwrap();
        let session_as_template = session_as_mutex.copy();
        let session_id = session_as_template.id;
        let session_as_json = object!{
            "session_id" => session_id
        };
        return Ok(resp::success_with_data(session_as_json));
    })

}

fn end(){
    //panic!("files actor closed");
    thread::spawn(move || {
        // Let's wait 20 milliseconds before notifying the condvar.
        thread::sleep(Duration::from_millis(3000));
        common::log("killing files actor");
        std::process::exit(1);
    });
}

pub fn kill(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|_body| {
        end();
        return Ok(resp::success());
    })

}
