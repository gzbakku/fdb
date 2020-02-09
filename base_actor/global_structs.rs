#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct Actor_Template {
    pub id:String,
    pub sig:String
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct Node_Template {
    pub id:String,
    pub sig:String,
    pub port:String
}

#[allow(dead_code)]
impl Node_Template {
    pub fn copy(&self) -> Node_Template {
        Node_Template {
            id:self.id.clone(),
            sig:self.sig.clone(),
            port:self.port.clone()
        }
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct Composer_Template {
    pub id:String,
    pub sig:String,
    pub ip:String,
    pub port:String
}

#[allow(dead_code)]
impl Composer_Template {
    pub fn copy(&self) -> Composer_Template {
        Composer_Template {
            id:self.id.clone(),
            sig:self.sig.clone(),
            ip:self.ip.clone(),
            port:self.port.clone()
        }
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct Session_Template {
    pub id:String,
    pub sig:String
}

impl Session_Template {
    pub fn copy(&self) -> Session_Template {
        Session_Template {
            id:self.id.clone(),
            sig:self.sig.clone()
        }
    }
}
