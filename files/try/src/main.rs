use postoffice::client;
use postoffice::client::{start_connection,common,channel};

mod add;
mod check;
mod get;
mod del;
// mod list;

fn main() {



    if false{
        let key = "8cfb30b34977529853bbe46afdbbd5ae".to_string();
        let connection_id = client::get_random_connection_id();
        let addr = "127.0.0.1:5200".to_string();
        match start_connection(&connection_id,addr,key) {
            Ok(_)=>{
                println!("connection established");
            },
            Err(_)=>{
                common::error("failed start connection");
            }
        }
    }
    if true{
        add_member("one", "filer", "127.0.0.1:5711");        //filer
        // add_member("one_filer", "filer", "127.0.0.1:5701");           //warehouse
    }

    if false{
        add::init();
    }
    if false{
        check::init();
    }
    if false{
        get::init();
    }
    if true{
        del::init();
    }

    // if false{
    //     list::init(&connection_id);
    // }

}

fn add_member(member_name:&'static str,channel_name:&'static str,address:&'static str){
    match channel::add_member(
        &channel_name.to_string(),
        &member_name.to_string(),
        &address.to_string(),
        &"2dc6bb40e73417bba878d3c8e3e08780".to_string()
    ){
        Ok(_)=>{},
        Err(_)=>{}
    }
}
