use json::JsonValue;
use crate::collections;

#[derive(Debug,Clone)]
pub struct Collection {
    pub path:String,
    pub ccid:String,
    pub cuid:String,
    pub files:Vec<String>
}

#[derive(Debug,Clone)]
pub struct Collection_ID {
    pub ccid:String,
    pub cuid:String
}

pub fn process_collection(collection:&JsonValue) -> Result<Collection,String> {

    let path:String;
    match collection["path"].as_str() {
        Some(str)=>{
            path = str.to_string();
        },
        None=>{
            return Err("failed-parse_path_to_string".to_string());
        }
    }

    let ids:Collection_ID;
    match parse_path(&path) {
        Ok(i)=>{
            ids = i;
        },
        Err(e)=>{
            let error = format!("failed-parse_path=>{}",e);
            return Err(error);
        }
    }

    let mut build = Collection {
        path:path.clone(),
        ccid:ids.ccid.clone(),
        cuid:ids.cuid.clone(),
        files:Vec::new()
    };

    match collections::ensure(&build.cuid,&build.ccid,&build.path) {
        Ok(_)=>{},
        Err(e)=>{
            let error = format!("collection_unensured=>{}",e);
            return Err(error);
        }
    }

    for file in collection["files"].members() {
        build.files.push(file.dump());
    }

    return Ok(build);

}

pub fn parse_path(path:&String) -> Result<Collection_ID,String> {

    if path.contains("/") == false {
        return Err("invalid_path".to_string());
    }

    let places = path.split("/").collect::<Vec<&str>>();
    let places_len = places.len();

    if places[0].len() == 0 || places[places_len - 1].len() == 0 {
        return Err("invalid_path-check_first/last-items_in_path".to_string());
    }

    let mut ccid = String::new();
    let mut index = 0;

    for place in places.iter() {
        let cal = index % 2;
        if cal == 0 || index == 0 {
            if places_len - 1 == index {
                ccid.push_str(&place.to_string());
            } else {
                ccid.push_str(&place.to_string());
                ccid.push_str("/xxx/");
            }
        }
        index += 1;
    }

    return Ok(Collection_ID {
        ccid:hash(ccid),
        cuid:hash(path.to_string())
    });

}

fn hash(m:String) -> String {
    let digest = md5::compute(m.as_bytes());
    format!("{:?}",digest)
}
