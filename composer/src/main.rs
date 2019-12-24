extern crate clap;
use clap::{Arg, App};

use std::format;

mod init;
mod common;
mod crypt;
mod io;

mod composer;
mod node;
mod starter;

/*

//-------------
//init fdb

//cargo run -- -p=akku -d=d://workstation/expo/rust/fdb/instance -c=d://workstation/expo/rust/fdb/instance/fdb.json --init

*/

/*

//-------------
//make new node

cargo run -- -p=akku --ip=103.214.61.242 --init --node --config=d://workstation/expo/rust/fdb/instance/fdb.json -d=d://workstation/expo/rust/fdb/ --out=d://workstation/expo/rust/fdb/this_node_config.json

*/

fn main() {

    let current_dir = io::current_dir();

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
                                  Arg::with_name("out")
                                   .help("output file path and extension ex :- d://example.json")
                                   .short("o")
                                   .long("out")
                                   .value_name("out")
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
        if matches.is_present("config") {
            match matches.value_of("config") {
                Some(v) => {
                    config_file_location = v.to_string();
                },
                None => {}
            }
        }

        let mut does_config_file_exists = false;
        if io::check_path(&config_file_location) {
            does_config_file_exists = true;
        }

        //***********************
        //extract ip address

        let mut ip = String::new();
        let mut given_by_user_ip = false;
        if matches.is_present("ip") {
            common::log("setting up given Ip Address");
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
        if io::check_path(&base_dir_location) {
            does_base_dir_exists = true;
        }

        //***********************
        //extract out dir

        let mut out_dir_location = format!("{}/instance/nodes/",current_dir.to_string());
        let mut given_by_user_out_dir = false;
        if matches.is_present("out") {
            common::log("setting up out file");
            match matches.value_of("out") {
                Some(v) => {
                    out_dir_location = v.to_string();
                    given_by_user_out_dir = true;
                },
                None => {}
            }
        }

        let mut does_out_dir_exists = false;
        if io::check_path(&out_dir_location) {
            does_out_dir_exists = true;
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
                common::error("you cannot init or reset at the same time.");
                return;
            }
            do_reset = true;
        }

        let mut do_make_new_node = false;
        if matches.is_present("node") {
            if do_init == false {
                common::error("you cannot make a new node without the init flag");
                return;
            }
            do_make_new_node = true;
        }

        if do_make_new_node {
            common::log("starting generate node");
            if given_by_user_out_dir == false {
                common::error("please provide output file path using --out=D://example.json flag.");
                return;
            }
            if does_out_dir_exists == true {
                println!("out_dir_location : {:?}",out_dir_location);
                common::error("given path already exists please remove the file or try a diffrent file name");
                return;
            }
            if does_config_file_exists == false {
                println!("config_file_location : {:?}",config_file_location);
                common::error("no config file found.");
                return;
            }
            if !given_by_user_ip {
                common::error("please provide a publically accessible ip address on which this node will be initiated.");
                return;
            }
            if given_by_user_base_dir {
                init::node(&ip,&password,&base_dir_location,&config_file_location,&out_dir_location);
                return;
            } else {
                common::error("please provide a base directory for the node to store and process files in this directory will be used on the host hardware file system so be carefull and make sure the file permissions are settuped correctly.");
                return;
            }
        }

        //init new database instance here
        if do_init {
            common::log("starting init");
            if does_config_file_exists {
                common::error("config file already exists in this location please choose a diffrent directory.");
                return;
            }
            if does_base_dir_exists {
                common::error("directory named fdb already exists please choose a diffrent directory to initiate a new fdb instance.");
                return;
            }
            init::init(&ip,&password,&base_dir_location,&config_file_location);
            return;
        }

        //reset the db instance here
        if do_reset {
            common::log("starting reset");
            if does_config_file_exists == false {
                common::error("no config file found.");
                return;
            }
            if does_base_dir_exists == false {
                common::error("no fdb base directory found.");
                return;
            }
            init::reset(&ip,&password,&base_dir_location,&config_file_location);
            return;
        }

        //start checks
        if does_config_file_exists == false {
            common::error("fdb config file not found in this directory you can set config file path via --config=d://fdb.json flag.");
            return;
        }
        if does_base_dir_exists == false {
            common::error("base directory does not exists");
            return;
        }

        match io::read_config(config_file_location,password) {
            Ok(r) => {
                let config = &r.config;
                if config.has_key("app") == false {
                    common::error("not a fdb config file. No Key Named App Found.");
                    return;
                }
                if config["app"].to_string() != "fdb".to_string() {
                    common::error("not a fdb config file. App Named Other then FDB.");
                    return;
                }
                if config.has_key("type") == false {
                    common::error("not a fdb config file. No Key Named App Found.");
                    return;
                }
                if
                    config["type"].to_string() != "node".to_string() &&
                    config["type"].to_string() != "composer".to_string()
                {
                    common::error("not a fdb config file. Invalid App Type node/composer.");
                    return;
                }
                let session = common::uid();
                if config["type"].to_string() == "node".to_string() {
                    node::init(&session,&r);
                }
                if config["type"].to_string() == "composer".to_string() {
                    composer::init(&session,&r);
                }
            },
            Err(_) => {
                common::error("failed to read config file, please make sure its a valid fdb config json file and a valid json object.");
                return;
            }
        }

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
