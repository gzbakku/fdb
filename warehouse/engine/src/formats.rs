use postoffice::check::{Format,Field,check};
use json::{JsonValue,parse};

#[derive(Debug,Clone)]
pub struct Act{
    pub func:String,
    pub file_name:String,
    pub file_type:String,
    pub item_index:String,
    pub start_index:u128,
    pub end_index:u128,
    pub item_value:String,
    pub items:Vec<String>
}

pub fn parse_activity(line:&String) -> Result<Act,&'static str> {

    let parsed:JsonValue;
    match parse(&line){
        Ok(obj)=>{
            parsed = obj;
        },
        Err(_)=>{
            return Err("failed-parse_to_json-parse_activity-formats");
        }
    }

    match check(&parsed,Format::builder(vec![
        Field::new("string",false,"type",vec![
        "add_item","delete_item","get_item","get_items","get_range",
        "delete_file","check_file","get_file","add_file",
        "list_dir"
        ],Field::no_format(),0,0,false),
        Field::new("object",false,"data",Field::no_options(),Field::no_format(),0,0,false)
    ])){
        Ok(_)=>{},
        Err(_)=>{
            return Err("failed-check_data");
        }
    }

    let format:Format;
    if parsed["type"] == "add_item"{
        format = item::add();
    } else if parsed["type"] == "delete_item"{
        format = item::delete();
    } else if parsed["type"] == "get_item"{
        format = item::get();
    } else if parsed["type"] == "get_items"{
        format = item::get_items();
    } else if parsed["type"] == "get_range"{
        format = item::get_range();
    } else if parsed["type"] == "add_file"{
        format = file::add();
    } else if parsed["type"] == "delete_file"{
        format = file::delete();
    } else if parsed["type"] == "get_file"{
        format = file::get();
    } else if parsed["type"] == "list_dir"{
        format = file::list_dir();
    } else {
        return Err("invalid-endpoint");
    }

    let data = &parsed["data"];
    match check(data,format){
        Ok(_)=>{},
        Err(_)=>{
            return Err("failed-check_data");
        }
    }

    //extract type, file type, file name and item index

    let func:String;
    match parsed["type"].as_str(){
        Some(str)=>{
            func = str.to_string();
        },
        None=>{
            return Err("failed-extract-type");
        }
    }

    let mut file_name:String = String::new();
    if parsed["type"] != "list_dir"{
        match parsed["data"]["file_name"].as_str(){
            Some(str)=>{
                file_name = str.to_string();
            },
            None=>{
                return Err("failed-extract-file_name");
            }
        }
    }

    let file_type:String;
    match parsed["data"]["file_type"].as_str(){
        Some(str)=>{
            file_type = str.to_string();
        },
        None=>{
            return Err("failed-extract-file_type");
        }
    }

    let mut item_index:String = "0".to_string();
    if
        parsed["type"] == "add_item" ||
        parsed["type"] == "get_item" ||
        parsed["type"] == "delete_item"
    {
        match parsed["data"]["item_index"].as_str(){
            Some(str)=>{
                item_index = str.to_string();
            },
            None=>{
                return Err("failed-extract-item_index");
            }
        }
    }

    let mut start_index:u128 = 0;
    if parsed["type"] == "get_range"{
        match parsed["data"]["start_index"].as_str(){
            Some(str)=>{
                match str.parse::<u128>(){
                    Ok(num)=>{
                        start_index = num;
                    },
                    Err(_)=>{
                        return Err("failed-parse-start_index");
                    }
                }
            },
            None=>{
                return Err("failed-extract-start_index");
            }
        }
    }

    let mut end_index:u128 = 0;
    if parsed["type"] == "get_range"{
        match parsed["data"]["end_index"].as_str(){
            Some(str)=>{
                match str.parse::<u128>(){
                    Ok(num)=>{
                        end_index = num;
                    },
                    Err(_)=>{
                        return Err("failed-parse-end_index");
                    }
                }
            },
            None=>{
                return Err("failed-extract-end_index");
            }
        }
    }

    let mut item_value:String = String::new();
    if
        parsed["type"] == "add_item" ||
        parsed["type"] == "add_file"
    {
        match parsed["data"]["item_value"].as_str(){
            Some(str)=>{
                item_value = str.to_string();
            },
            None=>{
                return Err("failed-extract-file_type");
            }
        }
    }

    let mut items:Vec<String> = Vec::new();
    if
        parsed["type"] == "get_items"
    {
        for item in parsed["data"]["items"].members(){
            match item.as_str(){
                Some(str)=>{
                    items.push(str.to_string());
                },
                None=>{
                    return Err("failed-extract-file_type");
                }
            }
        }
    }

    let build = Act{
        func:func,
        file_name:file_name,
        file_type:file_type,
        item_index:item_index,
        start_index:start_index,
        end_index:end_index,
        item_value:item_value,
        items:items
    };

    return Ok(build);

}

pub mod item{

    use postoffice::check::{Format,Field};

    pub fn add() -> Format {
        Format::builder(vec![
            Field::new("string",false,"file_name",Field::no_options(),Field::no_format(),0,0,false),
            Field::new("string",false,"file_type",Field::no_options(),Field::no_format(),0,0,false),
            Field::new("string",false,"item_index",Field::no_options(),Field::no_format(),0,0,false),
            Field::new("string",false,"item_value",Field::no_options(),Field::no_format(),0,0,false)
        ])
    }

    pub fn delete() -> Format {
        Format::builder(vec![
            Field::new("string",false,"file_name",Field::no_options(),Field::no_format(),0,0,false),
            Field::new("string",false,"file_type",Field::no_options(),Field::no_format(),0,0,false),
            Field::new("string",false,"item_index",Field::no_options(),Field::no_format(),0,0,false)
        ])
    }

    pub fn get_range() -> Format {
        Format::builder(vec![
            Field::new("string",false,"file_name",Field::no_options(),Field::no_format(),0,0,false),
            Field::new("string",false,"file_type",Field::no_options(),Field::no_format(),0,0,false),
            Field::new("string",false,"start_index",Field::no_options(),Field::no_format(),0,0,false),
            Field::new("string",false,"end_index",Field::no_options(),Field::no_format(),0,0,false)
        ])
    }

    pub fn get_items() -> Format {
        Format::builder(vec![
            Field::new("string",false,"file_name",Field::no_options(),Field::no_format(),0,0,false),
            Field::new("string",false,"file_type",Field::no_options(),Field::no_format(),0,0,false),
            Field::new("array",false,"items",Field::no_options(),Field::no_format(),0,0,false),
        ])
    }

    pub fn get() -> Format {
        Format::builder(vec![
            Field::new("string",false,"file_name",Field::no_options(),Field::no_format(),0,0,false),
            Field::new("string",false,"file_type",Field::no_options(),Field::no_format(),0,0,false),
            Field::new("string",false,"item_index",Field::no_options(),Field::no_format(),0,0,false),
        ])
    }

}

pub mod file{

    use postoffice::check::{Format,Field};

    pub fn add() -> Format {
        Format::builder(vec![
            Field::new("string",false,"file_name",Field::no_options(),Field::no_format(),0,0,false),
            Field::new("string",false,"file_type",Field::no_options(),Field::no_format(),0,0,false),
            Field::new("string",false,"data",Field::no_options(),Field::no_format(),0,0,false)
        ])
    }

    pub fn delete() -> Format {
        Format::builder(vec![
            Field::new("string",false,"file_name",Field::no_options(),Field::no_format(),0,0,false),
            Field::new("string",false,"file_type",Field::no_options(),Field::no_format(),0,0,false)
        ])
    }

    pub fn get() -> Format {
        Format::builder(vec![
            Field::new("string",false,"file_name",Field::no_options(),Field::no_format(),0,0,false),
            Field::new("string",false,"file_type",Field::no_options(),Field::no_format(),0,0,false)
        ])
    }

    pub fn list_dir() -> Format {
        Format::builder(vec![
            Field::new("string",false,"file_type",Field::no_options(),Field::no_format(),0,0,false)
        ])
    }

}
