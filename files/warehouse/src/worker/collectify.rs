use std::fs::File;
use std::collections::HashMap;
use postoffice::collector::collection::Collection;
use crate::formats::{parse_activity,Act};
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug,Clone)]
pub struct FdbFile{
    pub file_type:String,
    pub file_name:String,
    pub acts:Vec<Act>
}

pub fn init(collection:&mut Collection) -> Result<HashMap<String,FdbFile>,&'static str>{

    let reader:BufReader<File>;
    match collection.reader(){
        Ok(worker)=>{
            reader = worker;
        },
        Err(e)=>{
            println!("{:?}",e);
            return Err("failed-get_collection_reader-process_collector-writer");
        }
    }

    let mut files: HashMap<String, FdbFile> = HashMap::new();

    for line in reader.lines(){
        match line{
            Ok(str)=>{
                match parse_activity(&str){
                    Ok(parser)=>{
                        if !files.contains_key(&parser.file_name.clone()){
                            match files.insert(parser.file_name.clone(),FdbFile{
                                file_type:parser.file_type.clone(),
                                file_name:parser.file_name.clone(),
                                acts:Vec::new()
                            }){
                                Some(_)=>{},
                                None=>{}
                            }
                        }
                        match files.get_mut(&parser.file_name){
                            Some(worker)=>{
                                worker.acts.push(parser);
                            },
                            None=>{
                                return Err("failed-add_act-process_collector-writer");
                            }
                        }
                    },
                    Err(_)=>{
                        return Err("failed-parse_line_of_collection-process_collector-writer");
                    }
                }
            },
            Err(_)=>{
                return Err("failed-read_line-process_collector-writer");
            }
        }
    }

    return Ok(files);

}
