use crate::io::Extracted;
use crate::common;

pub fn init(e:&Extracted){

    common::log("initiating composer");

    //start the central server
    crate::starter::init(&e);

    //start the actors


}
