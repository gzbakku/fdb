extern crate clap;
use clap::{Arg, App, SubCommand};

use std::fs::{File,create_dir_all};
use std::path::Path;
use std::format;
use std::env;

mod init;
mod common;
mod crypt;

//cargo run -- -p=akku -d=d://workstation/expo/rust/fdb/instance -c=d://workstation/expo/rust/fdb/instance/fdb.json --init

//cargo run -- -p=akku -d=d://workstation/expo/rust/fdb/instance --ip=103.214.61.242 --init --node

fn main() {

    let current_dir_object = env::current_dir().unwrap();
    // let unparsed_current_dir = current_dir_object.to_str().unwrap();
    // let current_dir = unparsed_current_dir.replace("\\","/");
    let current_dir = current_dir_object.to_str().unwrap();

    common::log("starting fdb");

    let matches = App::new("Fuc* DB Composer")
                          .version("0.0.1")
                          .author("gzbakku. <gzbakku@gmail.com>")
                          .about("Fuc* DB Fastest NoSql Secure Database Written in Rust")
                          .arg(
                              Arg::with_name("password")
                               .help("password protected data access")
                               .short("p")
                               .long("password")
                               .value_name("password")
                               .required(true)
                           )
                          .arg(
                              Arg::with_name("init")
                               .help("config and init the database for the first time")
                               .short("i")
                               .long("init")
                               .required(false)
                           )
                           .arg(
                               Arg::with_name("node")
                                .help("congiure a new instance node use this flag in combination with init, base_dir, and password flags")
                                .long("node")
                                .required(false)
                            )
                           .arg(
                               Arg::with_name("reset")
                                .help("reset the db config.")
                                .short("r")
                                .long("reset")
                                .required(false)
                            )
                            .arg(
                                Arg::with_name("base_dir")
                                 .help("reset the db config.")
                                 .short("d")
                                 .long("dir")
                                 .value_name("base_dir")
                                 .required(false)
                             )
                             .arg(
                                 Arg::with_name("config")
                                  .help("location of config file")
                                  .short("c")
                                  .long("config")
                                  .value_name("config")
                                  .required(false)
                              )
                              .arg(
                                  Arg::with_name("ip")
                                   .help("your publically accessible ip address")
                                   .long("ip")
                                   .value_name("ip")
                                   .required(false)
                               )
                          .get_matches();

        //***********************
        //extract password

        common::log("extracting password");

        let mut password = String::new();
        let mut found_password = false;

        if matches.is_present("password") {
            match matches.value_of("password") {
                Some(v) => {
                    password = v.to_string();
                    found_password = true;
                },
                None => {}
            }
        }

        if found_password == false {
            println!("!!! please provide a valid password");
            return
        }

        //***********************
        //extract config File

        common::log("setting up config file");

        let mut config_file_location = format!("{}/instance/fdb.json",current_dir.to_string());
        let mut given_by_user_config = false;
        if matches.is_present("config") {
            match matches.value_of("config") {
                Some(v) => {
                    config_file_location = v.to_string();
                    given_by_user_config = true;
                },
                None => {}
            }
        }

        let mut does_config_file_exists = false;
        if Path::new(&config_file_location).exists() {
            does_config_file_exists = true;
        }

        //***********************
        //extract ip address

        common::log("setting up base directory");

        let mut ip = String::new();
        let mut given_by_user_ip = false;
        if matches.is_present("ip") {
            match matches.value_of("ip") {
                Some(v) => {
                    ip = v.to_string();
                    given_by_user_ip = true;
                },
                None => {}
            }
        }

        //***********************
        //extract base dir

        common::log("setting up base directory");

        let mut base_dir_location = format!("{}/instance/",current_dir.to_string());
        let mut given_by_user_base_dir = false;
        if matches.is_present("base_dir") {
            match matches.value_of("base_dir") {
                Some(v) => {
                    base_dir_location = v.to_string();
                    given_by_user_base_dir = true;
                },
                None => {}
            }
        }

        let mut does_base_dir_exists = false;
        if Path::new(&base_dir_location).exists() {
            does_base_dir_exists = true;
        }

        //***********************
        //run setup functions here

        let mut do_init = false;
        if matches.is_present("init") {
            do_init = true;
        }

        let mut do_reset = false;
        if matches.is_present("reset") {
            if do_init {
                println!("!!! you cannot init or reset at the same time.");
                return;
            }
            do_reset = true;
        }

        let mut do_make_new_node = false;
        if matches.is_present("node") {
            if do_init == false {
                println!("!!! you cannot make a new node without the init flag");
                return;
            }
            do_make_new_node = true;
        }

        if do_make_new_node {
            if does_config_file_exists == false {
                println!("{:?}",config_file_location);
                println!("!!! no config file found.");
                return;
            }
            if does_base_dir_exists == false {
                println!("!!! no fdb base directory found.");
                return;
            }
            if !given_by_user_ip {
                println!("!!! please provide a publically accessible ip address on which this node will be initiated.");
                return;
            }
            if !given_by_user_base_dir {
                println!("!!! please provide the base directory in which fdb will process and store the instance sensitive data.");
                return;
            }
            init::node(&ip,&password,&base_dir_location,&config_file_location);
            return;
        }

        //init new database instance here
        if do_init {
            if does_config_file_exists {
                println!("!!! config file already exists in this location please choose a diffrent directory.");
                return;
            }
            if does_base_dir_exists {
                println!("!!! directory named fdb already exists please choose a diffrent directory to initiate a new fdb instance.");
                return;
            }
            init::init(&ip,&password,&base_dir_location,&config_file_location);
        }

        //reset the db instance here
        if do_reset {
            if does_config_file_exists == false {
                println!("!!! no config file found.");
                return;
            }
            if does_base_dir_exists == false {
                println!("!!! no fdb base directory found.");
                return;
            }
            init::reset(&ip,&password,&base_dir_location,&config_file_location);
        }


}

fn check_setup(){

    let current_dir_object = env::current_dir().unwrap();
    let current_dir = current_dir_object.to_str().unwrap();

    let base_location = format!("{}/instance",current_dir);

    let location = format!("{}/instance/config.fdbv",current_dir);

    if Path::new(&location).exists() == false {

        match create_dir_all(&base_location) {
            Ok(_r) => {},
            Err(e) => {
                //return Err(e.to_string());
            }
        }

        //ask for password

    }

    let open = File::open(&location);

    println!("Hello, world!");

}

/******************************************************************

common protocol
----------------------------------------

check if base dir exists

check if config file exists

******************************************************************/

/******************************************************************

procedure for base composer
----------------------------------------

//take the password

//decrypt config file

//parse as json

//start the composing server

    //make the base key distributer

    //make the service chart distributer

//start the appropriate services with file input

//connect to foreign composers

******************************************************************/

/******************************************************************

procedure for foreign composer
----------------------------------------

//start the appropriate services with file input

******************************************************************/
