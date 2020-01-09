use crate::{issuer,subject};

#[derive(Debug, Clone)]
pub struct Builder {
    pub issuer:issuer::Issuer,
    pub subject:subject::Subject,
    pub certificate_path:String,
    pub key_path:String,
    pub key_size:u32
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            issuer:issuer::Issuer::new(),
            subject:subject::Subject::new(),
            certificate_path:String::from("null"),
            key_path:String::from("null"),
            key_size:1028
        }
    }
    pub fn set_certificate_path(&mut self,certificate_path:String){
        self.certificate_path = certificate_path;
    }
    pub fn set_key_path(&mut self,key_path:String){
        self.key_path = key_path;
    }
    pub fn set_key_size(&mut self,key_size:u32){
        self.key_size = key_size;
    }
}
