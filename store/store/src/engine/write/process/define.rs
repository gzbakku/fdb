use crate::engine::write::parse::Collection;

pub enum WriteResult {
    Error(ERROR),
    Success(SUCCESS)
}

pub struct SUCCESS{
    pub collection:String
}

impl SUCCESS {
    pub fn new(c:Collection) -> SUCCESS {
        SUCCESS {
            collection:c.cuid.clone()
        }
    }
}

pub struct ERROR{
    pub collection:String,
    pub error:String
}

impl ERROR {
    pub fn new(c:Collection,e:String) -> ERROR {
        ERROR {
            collection:c.cuid.clone(),
            error:e
        }
    }
    pub fn new_str(c:Collection,e:&str) -> ERROR {
        ERROR {
            collection:c.cuid.clone(),
            error:e.to_string()
        }
    }
}
