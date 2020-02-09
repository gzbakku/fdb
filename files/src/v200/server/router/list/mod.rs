use actix_web::{Error, HttpResponse};
use crate::{resp,DIR,io};
use bytes::Bytes;
use json::JsonValue;

pub async fn list(body: Bytes) -> Result<HttpResponse, Error> {

    let result = json::parse(std::str::from_utf8(&body).unwrap());
    let injson: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string() },
    };

    if &injson.has_key("location") == &false {
        return Ok(resp::error("not_found-file_name-list".to_string()));
    }
    if &injson["location"].is_string() == &false {
        return Ok(resp::error("invalid-file_data-list".to_string()));
    }

    let location = &injson["location"].to_string();

    if
        location != &"files".to_string() &&
        location != &"vault".to_string() &&
        location != &"list".to_string()
    {
        return Ok(resp::error("invalid-location-list".to_string()));
    }

    let get_dir = DIR.lock().unwrap()[0].to_string();

    let fetch = io::get_files(get_dir,location.to_string());
    match fetch {
        Ok(r) => {
            return Ok(resp::success_with_data(r));
        },
        Err(e) => {
            return Ok(resp::error(e.to_string()));
        }
    }

}
