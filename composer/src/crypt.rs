extern crate rand;

use aes_gcm::Aes256Gcm; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray};
use std::str;

//sample key = 8cfb30b34977529853bbe46afdbbd5ae
//sample iv/nonce = /B?E(H+MbQeT

//key length 32
//nonce length 12

use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Result {
    pub nonce:Vec<u8>,
    pub cipher:Vec<u8>
}

pub fn encrypt(i_data:String,i_key:String) -> Result {

    let key = GenericArray::clone_from_slice(i_key.as_bytes());
    let aead = Aes256Gcm::new(key);

    let my_data = i_data.as_bytes();

    let mut iv: [u8; 12] = [0; 12];
    let mut rng = thread_rng();
    rng.fill(&mut iv);

    let nonce = GenericArray::from_slice(&iv); // 96-bits; unique per message
    let ciphertext = aead.encrypt(nonce, my_data.as_ref()).expect("encryption failure!");

    return Result {
        nonce:iv.to_vec(),
        cipher:ciphertext
    };

}

pub fn decrypt(data:Vec<u8>,i_key:String,i_iv:Vec<u8>) -> String {

    let key = GenericArray::clone_from_slice(i_key.as_bytes());
    let aead = Aes256Gcm::new(key);
    let nonce = GenericArray::from_slice(&i_iv);

    let plaintext = aead.decrypt(nonce, data.as_ref()).expect("decryption failure!");

    let convert = str::from_utf8(&plaintext);

    let mut result = String::from("{}");

    match convert {
        Ok(v)=>{
            result = v.to_string();
        },
        Err(_e) => {}
    }

    return result;

}
