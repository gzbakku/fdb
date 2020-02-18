use crate::collector::io;

#[derive(Debug,Clone)]
pub struct Control {
    pub active:String,
    pub finished:Vec<String>
}

#[allow(dead_code)]
impl Control {
    #[allow(dead_code)]
    pub fn new() -> Control {
        Control {
            active:String::new(),
            finished:Vec::new()
        }
    }
    #[allow(dead_code)]
    pub fn overtake(self:&mut Self,base:Control){
        self.active = base.active;
        self.finished = base.finished;
    }
}

#[allow(dead_code)]
pub fn init(base:&String) -> Result<Control,String> {

    match essential_files(&base) {
        Ok(_)=>{},
        Err(_)=>{
            return Err("failed-make_essential_files".to_string());
        }
    }

    let control_path = format!("{}/control.fdbc",&base);
    match io::read(&control_path) {
        Ok(data)=>{
            match parse_control(data,base.to_string(),control_path) {
                Ok(base)=>{
                    return Ok(base);
                },
                Err(e)=>{
                    let error = format!("failed-parse_file=>{}",e);
                    return Err(error);
                }
            }
        },
        Err(_)=>{
            return Err("failed-read_file".to_string());
        }
    }

}

#[allow(dead_code)]
fn parse_control(data:String,base_dir:String,control_path:String) -> Result<Control,String> {

    if data.contains(";") == false {
        match make_new_control(base_dir,control_path) {
            Ok(base)=>{
                return Ok(base);
            },
            Err(e)=>{
                let error = format!("failed-make_new_control=>{}",e);
                return Err(error);
            }
        }
    }

    let hold = data.split(";").collect::<Vec<&str>>();

    let collection_vector = hold[1].split(",").collect::<Vec<&str>>();
    let mut finished_vector = Vec::new();
    for coll in collection_vector {
        if coll.len() == 32 {
            finished_vector.push(coll.to_string());
        }
    }
    finished_vector.push(hold[0].to_string());
    match update_control(&base_dir, &control_path, finished_vector) {
        Ok(base)=>{
            return Ok(base);
        },
        Err(e)=>{
            let error = format!("failed-update_control=>{}",e);
            return Err(error);
        }
    }

}

#[allow(dead_code)]
pub fn edit_control(control_path:String,control_string:String) -> Result<(),String> {
    if true {
        match io::write_control(control_path.to_string(), control_string.as_bytes().to_vec()) {
            Ok(_)=>{},
            Err(e)=>{
                let error = format!("failed-io-write_control=>{}",e);
                return Err(error);
            }
        }
    }
    return Ok(());
}

#[allow(dead_code)]
pub fn update_control(base_dir:&String,control_path:&String,finished:Vec<String>) -> Result<Control,String> {

    let mut collections = String::new();
    for coll in finished.clone() {
        if coll.len() > 0 {
            collections.push_str(&coll);
            collections.push_str(",");
        }
    }

    //make active collection
    let collector_file = io::get_random_file_name();
    let collector_file_path = format!("{}/{}.fdbcs",base_dir,collector_file);
    if io::new_file(&collector_file_path) == false {
        return Err("failed-create-collection_file-make_new_control".to_string());
    }

    //make collection vector
    let control_string = format!("{};{}",collector_file,collections);
    match io::write_control(control_path.to_string(), control_string.as_bytes().to_vec()) {
        Ok(_)=>{},
        Err(e)=>{
            let error = format!("failed-io-write_control=>{}",e);
            return Err(error);
        }
    }

    //return new base
    let control = Control {
        active:collector_file,
        finished:finished
    };

    return Ok(control);

}

#[allow(dead_code)]
pub fn make_new_control(base_dir:String,control_path:String) -> Result<Control,String> {
    let collector_file = io::get_random_file_name();
    let collector_file_path = format!("{}/{}.fdbcs",base_dir,collector_file);
    if io::new_file(&collector_file_path) == false {
        return Err("failed-create-collection_file-make_new_control".to_string());
    }
    let control_string = format!("{};",collector_file);
    let control = Control {
        active:collector_file,
        finished:Vec::new()
    };
    match io::write_control(control_path, control_string.as_bytes().to_vec()) {
        Ok(_)=>{
            return Ok(control);
        },
        Err(e)=>{
            let error = format!("failed-io-write_control=>{}",e);
            return Err(error);
        }
    }
}

#[allow(dead_code)]
pub fn essential_files(base:&String) -> Result<(),String> {
    let control_path = format!("{}/control.fdbc",&base);
    if io::check_path(&control_path) == false {
        if !io::new_file(&control_path) {
            return Err("failed-create_control_data_file".to_string());
        }
    }
    return Ok(());
}
