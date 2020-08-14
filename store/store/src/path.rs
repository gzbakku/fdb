use json::JsonValue;

#[allow(dead_code)]
pub fn check(path:&JsonValue) -> bool {
     if path.is_string() == false {
         return false;
     }
     match path.as_str() {
         Some(pather) =>{
             if parse(pather.to_string()) == false {
                 return false;
             }
         },
         None=>{
             return false;
         }
     }
     true
 }

#[allow(dead_code)]
 fn parse(path:String) -> bool {
     if path.contains("/") == false {
         return false;
     }
     let vec = path.split("/").collect::<Vec<&str>>();
     if vec[0].len() == 0 {
         return false;
     }
     let len = vec.len();
     if len == 1 {
         return true;
     } else if len == 2 {
         return false;
     }
     let cal = len % 2;
     if cal >= 1 {
         return true;
     } else {
         return false;
     }
 }
