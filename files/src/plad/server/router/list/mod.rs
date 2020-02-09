use actix_web::{web, Error, HttpResponse};
use futures::{Future, Stream};
use crate::{io,resp,DIR};
use json::JsonValue;

pub fn list(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|body| {

        let result = json::parse(std::str::from_utf8(&body).unwrap());
        let injson: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };

        if &injson.has_key("location") == &false {
            return Ok(resp::error("not_found-file_name-list"));
        }
        if &injson["location"].is_string() == &false {
            return Ok(resp::error("invalid-file_data-list"));
        }

        let location = &injson["location"].to_string();

        if
            location != &"files".to_string() &&
            location != &"vault".to_string() &&
            location != &"list".to_string()
        {
            return Ok(resp::error("invalid-location-list"));
        }

        let get_dir = DIR.lock().unwrap()[0].to_string();

        let fetch = io::get_files(get_dir,location.to_string());
        match fetch {
            Ok(r) => {
                return Ok(resp::success_with_data(r));
            },
            Err(e) => {
                println!("error : {:?}",e);
                return Ok(resp::error("failed-io_get_files"));
            }
        }

    })

}
