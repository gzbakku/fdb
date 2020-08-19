use postoffice::check::{Format,Field,check};
use json::{JsonValue,parse};

#[derive(Debug,Clone)]
pub struct Act{
    pub func:String,
    pub dir:String,
    pub index:String,
    pub value:String
}

pub fn parse_request(line:&String) -> Result<Act,&'static str> {

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
        "add","get","delete"
        ],Field::no_format(),0,0,false),
        Field::new("object",false,"data",Field::no_options(),Field::no_format(),0,0,false)
    ])){
        Ok(_)=>{},
        Err(_)=>{
            return Err("failed-check_data");
        }
    }

    let format:Format;
    if parsed["type"] == "add"{
        format = add();
    } else if parsed["type"] == "get"{
        format = get();
    } else if parsed["type"] == "delete"{
        format = delete();
    }  else {
        return Err("invalid-endpoint");
    }

    let data = &parsed["data"];
    match check(data,format){
        Ok(_)=>{},
        Err(_)=>{
            return Err("failed-check_data");
        }
    }

    //extract func, dir, index, value

    let func:String;
    match parsed["type"].as_str(){
        Some(str)=>{
            func = str.to_string();
        },
        None=>{
            return Err("failed-extract-type");
        }
    }

    let dir:String;
    match parsed["data"]["dir"].as_str(){
        Some(str)=>{
            dir = str.to_string();
        },
        None=>{
            return Err("failed-extract-index");
        }
    }

    let index:String;
    match parsed["data"]["index"].as_str(){
        Some(str)=>{
            index = str.to_string();
        },
        None=>{
            return Err("failed-extract-index");
        }
    }

    let mut value:String = String::new();
    if parsed["type"] == "add"{
        match parsed["data"]["value"].as_str(){
            Some(str)=>{
                value = str.to_string();
            },
            None=>{
                return Err("failed-extract-value");
            }
        }
    }

    let build = Act{
        func:func,
        dir:dir,
        index:index,
        value:value
    };

    return Ok(build);

}

fn add() -> Format {
    Format::builder(vec![
        Field::new("string",false,"dir",Field::no_options(),Field::no_format(),0,0,false),
        Field::new("string",false,"index",Field::no_options(),Field::no_format(),0,0,false),
        Field::new("string",false,"value",Field::no_options(),Field::no_format(),0,0,false)
    ])
}

fn get() -> Format {
    Format::builder(vec![
        Field::new("string",false,"dir",Field::no_options(),Field::no_format(),0,0,false),
        Field::new("string",false,"index",Field::no_options(),Field::no_format(),0,0,false),
    ])
}

fn delete() -> Format {
    Format::builder(vec![
        Field::new("string",false,"dir",Field::no_options(),Field::no_format(),0,0,false),
        Field::new("string",false,"index",Field::no_options(),Field::no_format(),0,0,false),
    ])
}
