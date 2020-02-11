extern crate rand;
use base64::{encode};
use aes_gcm::Aes256Gcm; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray};
use std::str;

//sample key = 8cfb30b34977529853bbe46afdbbd5ae
//sample iv/nonce = /B?E(H+MbQeT

//key length 32
//nonce length 12

use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct ENCRYPTED {
    pub nonce:Vec<u8>,
    pub cipher:Vec<u8>
}

#[allow(dead_code)]
pub fn encode_encrypt_message(message:String,key:String) -> String {
    let encrypted = encrypt(message,key);
    let nonce = encode(&encrypted.nonce);
    let cipher = encode(&encrypted.cipher);
    let message = format!("{}:{}",nonce,cipher);
    return message;
}

#[allow(dead_code)]
pub fn encrypt(i_data:String,i_key:String) -> ENCRYPTED {

    let key = GenericArray::clone_from_slice(i_key.as_bytes());
    let aead = Aes256Gcm::new(key);

    let my_data = i_data.as_bytes();

    let mut iv: [u8; 12] = [0; 12];
    let mut rng = thread_rng();
    rng.fill(&mut iv);

    let nonce = GenericArray::from_slice(&iv); // 96-bits; unique per message
    let ciphertext = aead.encrypt(nonce, my_data.as_ref()).expect("encryption failure!");

    return ENCRYPTED {
        nonce:iv.to_vec(),
        cipher:ciphertext
    };

}

pub fn decrypt(data:Vec<u8>,i_key:&Vec<u8>,i_iv:Vec<u8>) -> Result<String,String> {

    let key = GenericArray::clone_from_slice(i_key);
    let aead = Aes256Gcm::new(key);
    let nonce = GenericArray::from_slice(&i_iv);
    //let plaintext = aead.decrypt(nonce, data.as_ref()).expect("decryption failure!");

    match aead.decrypt(nonce, data.as_ref()) {
        Ok(plaintext)=>{
            match str::from_utf8(&plaintext) {
                Ok(v)=>{
                    return Ok(v.to_string());
                },
                Err(_e) => {
                    return Err("failed-to_convert-decrypt-crypt".to_string());
                }
            }
        },
        Err(_)=>{
            return Err("failed-to_decrypt-decrypt-crypt".to_string());
        }
    }



}
