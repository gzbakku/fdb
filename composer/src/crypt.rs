extern crate rand;

use aes_gcm::Aes256Gcm;
use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray};
use std::str;
use crate::common;

//key length 32
//nonce length 12

use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Encrypted {
    pub nonce:Vec<u8>,
    pub cipher:Vec<u8>
}

pub fn encrypt(i_data:String,i_key:String) -> Encrypted {

    let key = GenericArray::clone_from_slice(i_key.as_bytes());
    let aead = Aes256Gcm::new(key);

    let my_data = i_data.as_bytes();

    let mut iv: [u8; 12] = [0; 12];
    let mut rng = thread_rng();
    rng.fill(&mut iv);

    let nonce = GenericArray::from_slice(&iv); // 96-bits; unique per message
    let ciphertext = aead.encrypt(nonce, my_data.as_ref()).expect("encryption failure!");

    return Encrypted {
        nonce:iv.to_vec(),
        cipher:ciphertext
    };

}

pub fn decrypt(data:Vec<u8>,i_key:String,i_iv:Vec<u8>) -> Result<String,String> {

    let key = GenericArray::clone_from_slice(i_key.as_bytes());
    let aead = Aes256Gcm::new(key);
    let nonce = GenericArray::from_slice(&i_iv);

    match aead.decrypt(nonce, data.as_ref()) {
        Ok(r) => {
            let convert = str::from_utf8(&r);
            match convert {
                Ok(v)=>{
                    return Ok(v.to_string());
                },
                Err(_) => {
                    return Err("failed to parse cipher text to plain text".to_string());
                }
            }
        },
        Err(_) => {
            return Err(common::error("failed - decrypt secure password"));
        }
    }

}

pub fn extract_password(base_password:String,nonce:Vec<u8>,cipher:Vec<u8>) -> Result<String,String> {
    let hash = common::hash(base_password.to_string());
    match decrypt(cipher,hash.to_string(),nonce) {
        Ok(r) => {
            Ok(r)
        },
        Err(_) => {
            Err(common::error("password failed please try again."))
        }
    }
}
