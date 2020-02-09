use actix_web::{web, Error, HttpResponse};
use futures::{Future, Stream, Poll};
use crate::{io,resp,BOOK,DIR,KEY,crypt};
use json::JsonValue;

pub fn write(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|body| {

        let result = json::parse(std::str::from_utf8(&body).unwrap());
        let injson: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };

        if &injson.has_key("file") == &false {
            return Ok(resp::error("not_found-file_name".to_string()));
        }
        if &injson.has_key("data") == &false {
            return Ok(resp::error("not_found-file_data".to_string()));
        }
        if &injson["data"].is_object() == &false {
            return Ok(resp::error("invalid-file_data".to_string()));
        }

        let opener = &KEY.lock().unwrap();
        let key = &opener[0];

        let data_dump = &injson["data"].dump();
        let encrypted = crypt::encrypt(data_dump.to_string(),key.to_string());
        let combine = format!("{:?};{:?}",encrypted.nonce,encrypted.cipher);
        let get_dir = DIR.lock().unwrap()[0].to_string();

        match io::crypted::write(get_dir,injson["file"].to_string(),combine.as_bytes().to_vec()) {
            Ok(_r) => {},
            Err(e) => {
                return Ok(resp::error(e.to_string()));
            }
        }

        let open_book = &mut BOOK.lock().unwrap();
        open_book.insert(injson["file"].to_string(),injson["data"].clone());

        return Ok(resp::success());

    })

}

pub fn read(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|body| {

        let result = json::parse(std::str::from_utf8(&body).unwrap());
        let injson: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };

        if &injson.has_key("err") == &true {
            return Ok(resp::error("invalid_request-read_file".to_string()));
        }
        if &injson.has_key("file") == &false {
            return Ok(resp::error("not_found-read_file-file_name".to_string()));
        }

        //check book
        let file_name = &injson["file"].to_string();
        let open_book = &mut BOOK.lock().unwrap();
        if open_book.contains_key(file_name) {
            return Ok(resp::success_with_data(open_book[file_name].clone()));
        }

        //read from file
        let get_dir = DIR.lock().unwrap()[0].to_string();
        let result: io::crypted::CRYPT;
        match io::crypted::read(get_dir,file_name.to_string()) {
            Ok(r) => {
                result = r;
            },
            Err(e) => {
                return Ok(resp::error(e.to_string()));
            }
        }

        let opener = &KEY.lock().unwrap();
        let key = &opener[0];

        let decrypted = crypt::decrypt(result.cipher,key.to_string(),result.nonce);
        let result = json::parse(&decrypted);
        let data: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };

        if &data.has_key("err") == &true {
            return Ok(resp::error("failed-parse_decrypted_into_object".to_string()));
        }

        open_book.insert(file_name.to_string(),data.clone());

        return Ok(resp::success_with_data(data));

    })

}
