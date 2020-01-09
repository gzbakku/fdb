use openssl;
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;
use openssl::asn1::Asn1Time;

use std::io::prelude::*;
use std::fs::File;

pub mod issuer;
pub mod subject;
mod common;
pub mod builder;

use builder::Builder;

fn test(){

    let mut build = Builder::new();

    build.set_key_path("D://workstation/expo/rust/fdb/cert/keys/key.pem".to_string());
    build.set_certificate_path("D://workstation/expo/rust/fdb/cert/keys/cert.pem".to_string());
    build.set_key_size(4048);

    build.issuer.set_country("IN".to_string());
    build.issuer.set_state("UP".to_string());
    build.issuer.set_location("GZB".to_string());
    build.issuer.set_org("DAACHI".to_string());
    build.issuer.set_common_name("https://daachi.in".to_string());

    build.subject.set_country("IN".to_string());
    build.subject.set_state("UP".to_string());
    build.subject.set_location("GZB".to_string());
    build.subject.set_org("DAACHI".to_string());
    build.subject.set_common_name("127.0.0.1".to_string());

    //println!("build : {:?}",build);

    match create(&mut build) {
        Ok(_)=>{
            common::log("ssl files created successfully");
        },
        Err(_)=>{
            common::error("failed to create ssl files");
        }
    }

}

fn create(config:&mut builder::Builder) -> Result<(),String> {

    match openssl::rsa::Rsa::generate(config.key_size) {
        Ok(keys)=> {

            //***************************************************
            //rsa keys to PEM

            let private_key_as_u8_vec;
            match keys.private_key_to_pem() {
                Ok(r)=>{
                    private_key_as_u8_vec = r;
                },
                Err(_) => {
                    return Err(common::error("failed to create private key vector"));
                }
            }

            let public_key_as_u8_vec;
            match keys.public_key_to_pem() {
                Ok(r)=>{
                    public_key_as_u8_vec = r;
                },
                Err(e) => {
                    println!("error : {:?}",e);
                    return Err(common::error("failed to create public key vector"));
                }
            }

            //***************************************************
            //rsa to pKeyRef

            let public_key_ref;
            match PKey::public_key_from_pem(&public_key_as_u8_vec) {
                Ok(r)=>{
                    public_key_ref = r;
                },
                Err(e)=>{
                    println!("error : {:?}",e);
                    return Err(common::error("failed to create public key ref"));
                }
            }

            let private_key_ref;
            match PKey::private_key_from_pem(&private_key_as_u8_vec) {
                Ok(r)=>{
                    private_key_ref = r;
                },
                Err(e)=>{
                    println!("error : {:?}",e);
                    return Err(common::error("failed to create public key ref"));
                }
            }

            //***************************************************
            //make x509 cert

            //let mut x509 = openssl::x509::X509::builder().unwrap();

            let mut x509;
            match openssl::x509::X509::builder() {
                Ok(r)=>{
                    x509 = r;
                },
                Err(e)=>{
                    println!("error : {:?}",e);
                    return Err(common::error("failed-initiate-x509Builder"));
                }
            }

            match x509.set_pubkey(&public_key_ref) {
                Ok(_)=>{},
                Err(e)=>{
                    println!("error : {:?}",e);
                    return Err(common::error("failed-set_sub_key"));
                }
            }

            match openssl::x509::X509NameBuilder::new() {
                Ok(mut x509_name)=>{
                    let map = config.subject.to_hash_map();
                    for (key, val) in map.iter() {
                        match x509_name.append_entry_by_text(key, val) {
                            Ok(_)=>{},
                            Err(e)=>{
                                println!("error : {:?}",e);
                                println!("key : {:?} , val : {:?}",key,val);
                                return Err(common::error("failed-set_subject_row"));
                            }
                        }
                    }
                    let x509_name = x509_name.build();
                    match x509.set_subject_name(&x509_name) {
                        Ok(_)=>{},
                        Err(_)=>{
                            return Err(common::error("failed to set x509_name"));
                        }
                    }
                },
                Err(_)=>{
                    return Err(common::error("failed to initiate name builder for x509_name"));
                }
            }

            match openssl::x509::X509NameBuilder::new() {
                Ok(mut x509_issuer)=>{
                    let map = config.issuer.to_hash_map();
                    for (key, val) in map.iter() {
                        match x509_issuer.append_entry_by_text(key, val) {
                            Ok(_)=>{},
                            Err(e)=>{
                                println!("error : {:?}",e);
                                println!("key : {:?} , val : {:?}",key,val);
                                return Err(common::error("failed-set_issuer_row"));
                            }
                        }
                    }
                    let x509_issuer = x509_issuer.build();
                    match x509.set_issuer_name(&x509_issuer) {
                        Ok(_)=>{},
                        Err(_)=>{
                            return Err(common::error("failed to set x509_issuer"));
                        }
                    }
                },
                Err(_)=>{
                    return Err(common::error("failed to initiate name builder for x509_issuer"));
                }
            }

            match Asn1Time::days_from_now(365) {
                Ok(r)=>{
                    match x509.set_not_after(&r) {
                        Ok(_)=>{},
                        Err(e)=>{
                            println!("error : {:?}",e);
                            return Err(common::error("failed-set-Asn1Time_expiry_date"));
                        }
                    }
                },
                Err(e)=>{
                    println!("error : {:?}",e);
                    return Err(common::error("failed-get-Asn1Time_expiry_date"));
                }
            }

            match Asn1Time::days_from_now(0) {
                Ok(r)=>{
                    match x509.set_not_before(&r) {
                        Ok(_)=>{},
                        Err(e)=>{
                            println!("error : {:?}",e);
                            return Err(common::error("failed-set-Asn1Time_start_date"));
                        }
                    }
                },
                Err(e)=>{
                    println!("error : {:?}",e);
                    return Err(common::error("failed-get-Asn1Time_start_date"));
                }
            }

            if true {
                match x509.sign(&private_key_ref,MessageDigest::sha512()) {
                    Ok(_)=>{},
                    Err(e)=>{
                        println!("!!! : {:?}",e);
                        return Err(common::error("failed to sign x509 certificate"));
                    }
                }
            }

            let x509_cert = x509.build();

            let certificate_as_u8_vec;
            match x509_cert.to_pem() {
                Ok(r)=>{
                    certificate_as_u8_vec = r;
                },
                Err(_)=>{
                    return Err(common::error("failed to parse x509 certificate to u8 vector"));
                }
            }

            //***************************************************
            //write files

            if true {

                match make_file(config.certificate_path.clone(),certificate_as_u8_vec) {
                    Ok(_)=>{},
                    Err(_)=>{
                        return Err(common::error("failed to write x509 certificate"));
                    }
                }

                match make_file(config.key_path.clone(),private_key_as_u8_vec) {
                    Ok(_)=>{},
                    Err(_)=>{
                        return Err(common::error("failed to write rsa private key"));
                    }
                }

            }

            return Ok(());

        },
        Err(e) => {
            println!("error : {:?}",e);
            return Err(common::error("failed to create rsa keys"));
        }
    }

}

fn make_file(path:String,data:Vec<u8>) -> Result<(),String> {

    match File::create(&path) {
        Ok(mut r) => {
            match r.write(&data) {
                Ok(_)=>{
                    return Ok(());
                },
                Err(e)=>{
                    return Err(common::error_string(format!("!!! failed - write to file at => {:?} , Error => {:?}",&path,e)));
                }
            }
        },
        Err(e) => {
            return Err(common::error_string(format!("!!! failed - create to file at => {:?} , Error => {:?}",&path,e)));
        }
    }

}
