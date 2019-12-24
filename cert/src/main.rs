extern crate clap;
use clap::{Arg, App};

use openssl;
use std::env;

use std::io::prelude::*;
use std::fs::File;

fn main(){

    let current_dir_object = env::current_dir().unwrap();
    let current_dir = current_dir_object.to_str().unwrap();

    let matches = App::new("Fuc* DB Composer")
                          .version("0.0.1")
                          .author("gzbakku. <gzbakku@gmail.com>")
                          .about("Fuc* DB Fastest NoSql Secure Database Written in Rust")
                           .arg(
                               Arg::with_name("public")
                                .help("output file path and extension ex :- d://public.pem")
                                .long("public")
                                .value_name("public")
                                .required(false)
                            )
                            .arg(
                                Arg::with_name("private")
                                 .help("output file path and extension ex :- d://private.pem")
                                 .long("private")
                                 .value_name("private")
                                 .required(false)
                             )
                          .get_matches();

        //***********************
        //extract private file path

        let mut private_pem_file_path = format!("{}/private.pem",current_dir.to_string());
        if matches.is_present("private") {
            match matches.value_of("private") {
                Some(v) => {
                    private_pem_file_path = v.to_string();
                },
                None => {}
            }
        }

        //***********************
        //extract public file path

        let mut public_pem_file_path = format!("{}/public.pem",current_dir.to_string());
        if matches.is_present("private") {
            match matches.value_of("private") {
                Some(v) => {
                    public_pem_file_path = v.to_string();
                },
                None => {}
            }
        }

        create(private_pem_file_path,public_pem_file_path);

}

fn create(private_key_path:String,public_key_path:String) {

    match openssl::rsa::Rsa::generate(4048) {
        Ok(keys)=> {

            let private_key_as_u8_vec;
            let public_key_as_u8_vec;

            match keys.private_key_to_pem() {
                Ok(r)=>{
                    private_key_as_u8_vec = r;
                },
                Err(e) => {
                    println!("!!! failed to create private key error : {:?}",e);
                    return;
                }
            }

            match keys.public_key_to_pem() {
                Ok(r)=>{
                    public_key_as_u8_vec = r;
                },
                Err(e) => {
                    println!("!!! failed to create public key error : {:?}",e);
                    return;
                }
            }

            //make private.pem here
            match File::create(&private_key_path) {
                Ok(mut r) => {
                    match r.write(&private_key_as_u8_vec) {
                        Ok(_)=>{},
                        Err(e)=>{
                            println!("!!! failed - write to private.pem file at => {:?} , Error => {:?}",&private_key_path,e);
                            return;
                        }
                    }
                },
                Err(e) => {
                    println!("!!! failed - create private.pem file at => {:?} , Error => {:?}",&private_key_path,e);
                    return;
                }
            }

            //make public.pem here
            match File::create(&public_key_path) {
                Ok(mut r) => {
                    match r.write(&public_key_as_u8_vec) {
                        Ok(_)=>{
                            print!("Ok");
                        },
                        Err(e)=>{
                            println!("!!! failed - write to private.pem file at => {:?} , Error => {:?}",&public_key_path,e);
                            return;
                        }
                    }
                },
                Err(e) => {
                    println!("!!! failed - create private.pem file at => {:?} , Error => {:?}",&public_key_path,e);
                    return;
                }
            }

        },
        Err(e) => {
            println!("!!! failed - create rsa keys : {:?}",e);
            return;
        }
    }

}
