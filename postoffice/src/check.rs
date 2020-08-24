use json::JsonValue;
use crate::server::Request;
use crate::resp;

#[derive(Debug,Clone)]
pub struct Format {
    pub fields:Vec<Field>
}

impl Format {
    #[allow(dead_code)]
    pub fn new() -> Format {
        Format {
            fields:Vec::new()
        }
    }
    #[allow(dead_code)]
    pub fn builder(fields:Vec<Field>) -> Format {
        Format {
            fields:fields
        }
    }
    #[allow(dead_code)]
    pub fn new_field(self:&mut Self,field:Field){
        self.fields.push(field);
    }
    #[allow(dead_code)]
    pub fn field_builder(self:&mut Self,fields:Vec<Field>){
        for field_base in fields.iter() {
            let field = field_base.clone();
            self.fields.push(field);
        }
    }
}

#[derive(Debug,Clone)]
pub struct Field {
    pub r#type:String,
    pub elective:bool,
    pub key:String,
    pub options:Vec<String>,
    pub format:Format,
    pub min:u64,
    pub max:u64,
    pub check_limits:bool
}

impl Field {
    #[allow(dead_code)]
    pub fn new(data_type:&str,elective:bool,key:&str,options:Vec<&str>,format:Format,min:u64,max:u64,check_limits:bool) -> Field {
        let mut collect = Vec::new();
        for opt in options.iter() {
            collect.push(opt.to_string());
        }
        Field {
            r#type:data_type.to_string(),
            elective:elective,
            key:key.to_string(),
            options:collect,
            format:format,
            min:min,
            max:max,
            check_limits:check_limits
        }
    }
    #[allow(dead_code)]
    pub fn no_options() -> Vec<&'static str> {
        let hold:Vec<&str> = Vec::new();
        return hold;
    }
    #[allow(dead_code)]
    pub fn no_format() -> Format {
        return Format::new();
    }
}

#[allow(dead_code)]
fn make_error(e:&str,f:&Field) -> String {
    let error = format!("{} field : {:?}",e,f);
    return error;
}

#[allow(dead_code)]
fn make_error_str(e:String,f:&Field) -> String {
    let error = format!("{} field : {:?}",e,f);
    return error;
}


#[allow(dead_code)]
pub fn check_request(req:Request,format:Format) -> Result<JsonValue,String> {
    let body:JsonValue;
    match resp::get_body(&req) {
        Ok(parsed)=>{
            body = parsed;
        },
        Err(_)=>{
            return Err("failed to parse body".to_string());
        }
    }
    match check(&body,format) {
        Ok(_)=>{
            return Ok(body);
        },
        Err(e)=>{
            let error = format!("format check failed error : {:?}",e);
            return Err(error);
        }
    }
}

#[allow(dead_code)]
pub fn check(data:&JsonValue,format:Format) -> Result<(),String> {
    for field in format.fields.iter() {
        let base_key = field.key.clone();
        if data.has_key(&base_key.clone()) == false {
            return Err(make_error("not_found-key",field));
        }
        match check_field(&data[base_key],field) {
            Ok(_)=>{},
            Err(e)=>{
                return Err(make_error_str(e,field));
            }
        }

    }
    return Ok(());
}

#[allow(dead_code)]
pub fn check_children(data:&JsonValue,data_type:String,s_options:Vec<&str>,format:Format,check_format_on_array_items:bool,check_format_on_object:bool) -> Result<(),String> {
    let mut options = Vec::new();
    let base_options = s_options.clone();
    if s_options.len() > 1 {
        for opt in s_options.iter() {
            options.push(opt.to_string());
        }
    }
    for entry in data.entries() {
        let data = entry.1;
        match check_data_type(data, data_type.clone()) {
            Ok(_)=>{},
            Err(e)=>{
                return Err(e.to_string());
            }
        }
        if data_type == "string".to_string() && options.len() > 1 {
            match data.as_str() {
                Some(item)=>{
                    if options.contains(&item.to_string()) == false {
                        return Err("invalid-option-string".to_string());
                    }
                },
                None=>{
                    return Err("failed-extract_data-for_option_check".to_string());
                }
            }
        } else if data_type == "object".to_string() && check_format_on_object && format.fields.len() > 0 {
            match check(&data.clone(),format.clone()) {
                Ok(_)=>{},
                Err(e)=>{
                    let error = format!("failed-check_object-format=>{}",e);
                    return Err(error);
                }
            }
        } else if data_type == "array".to_string() && check_format_on_array_items {
            match check_array(data,String::from("object"),base_options.clone(),&format) {
                Ok(_)=>{},
                Err(_)=>{
                    return Err("failed-check_array_item_against_format".to_string());
                }
            }
        }
    }//loop the entries
    return Ok(());
}

#[allow(dead_code)]
pub fn check_array(data:&JsonValue,data_type:String,base_options:Vec<&str>,format:&Format) -> Result<(),String> {

    let mut options = Vec::new();
    if base_options.len() > 1 {
        for opt in base_options.iter() {
            options.push(opt.to_string());
        }
    }

    for member in data.members() {
        match check_data_type(member, data_type.clone()) {
            Ok(_)=>{},
            Err(e)=>{
                return Err(e.to_string());
            }
        }
        if data_type == "string".to_string() && options.len() > 1 {
            match data.as_str() {
                Some(item)=>{
                    if options.contains(&item.to_string()) == false {
                        return Err("invalid-option-string".to_string());
                    }
                },
                None=>{
                    return Err("failed-extract_data-for_option_check".to_string());
                }
            }
        } else if data_type == "object".to_string() && format.fields.len() > 0 {
            match check(&member,format.clone()) {
                Ok(_)=>{},
                Err(e)=>{
                    let error= format!("failed-check_object-format=>{}",e);
                    return Err(error);
                }
            }
        }
    }

    return Ok(());

}

#[allow(dead_code)]
pub fn check_field(data:&JsonValue,field:&Field) -> Result<(),String> {

    let data_type = field.r#type.clone();
    let options = field.options.clone();

    match check_data_type(data, data_type.clone()) {
        Ok(_)=>{},
        Err(e)=>{
            return Err(e);
        }
    }

    if field.check_limits {
        match check_limit(&data, data_type.clone(), field.min, field.max) {
            Ok(_)=>{},
            Err(e)=>{
                let error = format!("limits_exceeded=>{}",e);
                return Err(error);
            }
        }
    }

    if data_type == "string".to_string() {
        if options.len() > 1 {
            match data.as_str() {
                Some(item)=>{
                    if options.contains(&item.to_string()) == false {
                        return Err("invalid-option-string".to_string());
                    }
                },
                None=>{
                    return Err("failed-extract_data-for_option_check".to_string());
                }
            }
        }
    } else if data_type == "object".to_string() {
        if data.is_object() == false {
            return Err("invalid-data_type-object".to_string());
        }
        if field.format.fields.len() > 0 {
            match check(&data.clone(),field.format.clone()) {
                Ok(_)=>{},
                Err(e)=>{
                    let error = format!("failed-check_object-format=>{}",e);
                    return Err(error);
                }
            }
        }
    }

    return Ok(());

}

#[allow(dead_code)]
pub fn check_limit(data:&JsonValue,data_type:String,min:u64,max:u64) -> Result<(),String> {
    if data_type == "string".to_string() {
        match data.as_str(){
            Some(val)=>{
                let min_as_usize = u64_to_uzise(min);
                let max_as_usize = u64_to_uzise(max);
                let str_len = val.len();
                if str_len < min_as_usize || str_len > max_as_usize {
                    return Err("limits_reached-check_limit_for-string".to_string());
                }
            },
            None=>{
                return Err("failed-extract_data_as-string".to_string());
            }
        }
    } else if data_type == "number".to_string() {
        match data.as_u64(){
            Some(val)=>{
                if val < min || val > max {
                    return Err("limits_reached-check_limit_for-number".to_string());
                }
            },
            None=>{
                return Err("failed-extract_data_as-string".to_string());
            }
        }
    } else if data_type == "object".to_string() {
        let min_as_usize = u64_to_uzise(min);
        let max_as_usize = u64_to_uzise(max);
        let data_size = data.len();
        if data_size < min_as_usize || data_size > max_as_usize {
            return Err("limits_reached-check_limit_for-array".to_string());
        }
    } else if data_type == "array".to_string() {
        let min_as_usize = u64_to_uzise(min);
        let max_as_usize = u64_to_uzise(max);
        let data_size = data.len();
        if data_size < min_as_usize || data_size > max_as_usize {
            return Err("limits_reached-check_limit_for-array".to_string());
        }
    } else {
        let error = format!("invalid-data_type=>{}",data_type);
        return Err(error);
    }
    return Ok(());
}

#[allow(dead_code)]
pub fn u64_to_uzise(num:u64) -> usize {
    let hold = num.to_string();
    match hold.parse::<usize>() {
        Ok(val)=>{
            return val;
        },
        Err(_)=>{
            return 0;
        }
    }
}

#[allow(dead_code)]
pub fn check_data_type(data:&JsonValue,data_type:String) -> Result<(),String> {

    if data_type == "string".to_string() {
        if data.is_string() == false {
            return Err("invalid-data_type-string".to_string());
        }
    } else if data_type == "number".to_string() {
        if data.is_number() == false {
            return Err("invalid-data_type-number".to_string());
        }
    } else if data_type == "object".to_string() {
        if data.is_object() == false {
            return Err("invalid-data_type-object".to_string());
        }
    } else if data_type == "array".to_string() {
        if data.is_array() == false {
            println!("data : {:?}",&data);
            return Err("invalid-data_type-array".to_string());
        }
    } else if data_type == "boolean".to_string() || data_type == "bool".to_string() {
        if data.is_boolean() == false {
            return Err("invalid-data_type-boolean".to_string());
        }
    } else {
        let error = format!("invalid-data_type=>{}",data_type);
        return Err(error);
    }

    return Ok(());

}
