use actix_web::{web, Error, HttpResponse, HttpRequest};
use futures::{Future, Stream};
use crate::{io,resp,BOOK,DIR,KEY,crypt,common};
use json::{JsonValue,object,parse,from};

#[derive(Debug)]
struct File {
    name:String,
    collection:String,
    data:JsonValue
}

pub fn write(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|body| {

        let result = json::parse(std::str::from_utf8(&body).unwrap());
        let injson: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };

        if &injson.has_key("err") == &true {
            return Ok(resp::error("invalid_request-read_files"));
        }
        if &injson.has_key("files") == &false {
            return Ok(resp::error("not_found-read_files-file_name"));
        }
        if &injson["files"].is_array() == &false {
            return Ok(resp::error("files-not_an_array-read_files"));
        }

        let mut collect = Vec::new();
        for file in injson["files"].members() {
            match parse_file(file){
                Ok(parsed)=>{
                    collect.push(parsed);
                },
                Err(e)=>{
                    return Ok(resp::error(&e.to_string()));
                }
            }
        }

        let base_as_mutex = DIR.lock().unwrap();
        let base = base_as_mutex[0].clone();

        let mut success = JsonValue::new_array();
        let mut failed = JsonValue::new_array();

        for file in collect.iter() {
            match write_file(&file,&base) {
                Ok(_)=>{
                    success.push(file.name.clone());
                    &mu tBOOK.lock().unwrap().insert(file.name.clone(),file.data.clone());
                },
                Err(e)=>{
                    let mut failed_object = JsonValue::new_object();
                    failed_object.insert("error",e.to_string());
                    failed_object.insert("name",file.name.clone());
                    failed.push(failed_object);
                }
            }
        }

        let mut result = JsonValue::new_object();
        result.insert("failed",failed);
        result.insert("success",success);

        return Ok(resp::success_with_data(result));

    })

}

fn write_file(file:&File,base:&String) -> Result<(),String> {

    let dir_path = format!("{}/files/collections/{}",base,file.collection.clone());
    let file_path = format!("{}/{}.json",dir_path,&file.name.clone());

    if true {
        match io::write(dir_path,file_path,file.data.clone()) {
            Ok(_)=>{
                return Ok(());
            },
            Err(e)=>{
                return Err(e.to_string());
            }
        }
    }

    return Ok(());

}

fn parse_file(file:&JsonValue) -> Result<File,String> {

    if
        !file["name"].is_string() ||
        !file["collection"].is_string() ||
        !file["data"].is_object()
    {
        return Err(common::error("failed-check_file_params-parse_file"));
    }

    let mut build = File {
        name:String::new(),
        collection:String::new(),
        data:JsonValue::new_object()
    };

    match file["name"].as_str() {
        Some(name)=>{
            build.name = name.to_string();
        },
        None=>{
            return Err(common::error("failed-extract_name_param-parse_file"));
        }
    }

    match file["collection"].as_str() {
        Some(collection)=>{
            build.collection = collection.to_string();
        },
        None=>{
            return Err(common::error("failed-extract_collection_param-parse_file"));
        }
    }

    build.data = file["data"].clone();

    return Ok(build);

}

pub fn read(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {

    payload.concat2().from_err().and_then(|body| {

        let result = json::parse(std::str::from_utf8(&body).unwrap());
        let injson: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };

        if &injson.has_key("err") == &true {
            return Ok(resp::error("invalid_request-read_files"));
        }
        if &injson.has_key("files") == &false {
            return Ok(resp::error("not_found-read_files-file_name"));
        }
        if &injson.has_key("collection") == &false {
            return Ok(resp::error("not_found-read_files-collection"));
        }
        if &injson["files"].is_array() == &false {
            return Ok(resp::error("files-not_an_array-read_files"));
        }

        //check book
        let collection = &injson["collection"].to_string();


        //read from file
        let get_dir = DIR.lock().unwrap()[0].to_string();
        let collection_path = format!("{}/files/collections/{}",get_dir,collection);

        let mut result = JsonValue::new_object();
        let mut failed = JsonValue::new_array();
        let mut success = JsonValue::new_object();

        let open_book = &mut BOOK.lock().unwrap();

        for file in injson["files"].members() {
            match &file.as_str() {
                Some(file_name)=>{
                    let file_path = format!("{}/{}.json",collection_path,&file_name);
                    let book_mark = common::hash(format!("{}{}",collection,file_name));

                    let mut found_in_book = false;
                    if BOOK.lock().unwrap().contains_key(&book_mark) {
                        match BOOK.lock().unwrap().get("book_mark") {
                            Some(data)=>{
                                println!("read from book");
                                found_in_book = true;
                                success.insert(file_name,data.clone());
                            },
                            None=>{}
                        }
                    } else  {
                        match io::read(file_path) {
                            Ok(data_as_json) => {
                                success.insert(file_name,data_as_json.clone());
                                &mut BOOK.lock().unwrap().insert(book_mark,data_as_json);
                            },
                            Err(e) => {
                                common::error("failed-read_file-io-read-mod");
                                failed.push(file.dump());
                            }
                        }
                    }//fetch file gtom io

                },
                None=>{
                    common::error("failed-parse_files_array-read-mod");
                    failed.push(file.dump());
                }
            }//convert filanem to string
        }//loop the files array

        result.insert("failed",failed);
        result.insert("success",success);

        return Ok(resp::success_with_data(result));

    })

}
