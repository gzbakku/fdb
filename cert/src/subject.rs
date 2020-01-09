
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Subject {
    pub common_name:String,
    pub org:String,
    pub location:String,
    pub state:String,
    pub country:String
}

impl Subject {
    pub fn new() -> Subject {
        Subject {
            country:String::new(),
            org:String::new(),
            location:String::new(),
            state:String::new(),
            common_name:String::new(),
        }
    }
    pub fn set_country(&mut self,country:String){
        self.country = country;
    }
    pub fn set_org(&mut self,org:String){
        self.org = org;
    }
    pub fn set_location(&mut self,location:String){
        self.location = location;
    }
    pub fn set_state(&mut self,state:String){
        self.state = state;
    }
    pub fn set_common_name(&mut self,common_name:String){
        self.common_name = common_name;
    }
    pub fn to_hash_map(&mut self) -> HashMap<String,String> {
        let mut collect = HashMap::new();
        collect.insert("C".to_string(),self.country.to_string());
        collect.insert("ST".to_string(),self.state.to_string());
        collect.insert("L".to_string(),self.location.to_string());
        collect.insert("O".to_string(),self.org.to_string());
        collect.insert("CN".to_string(),self.common_name.to_string());
        return collect;
    }
}
