use std::{ any::Any, str::Utf8Error};
use aes_gcm::{aead::{Aead, AeadMutInPlace}, Aes256Gcm, KeyInit, Nonce};
use hmac::{Hmac};
use pbkdf2::pbkdf2;
use sha2::{Sha512};
use aes_gcm::{aead::{Payload}, Key}; 
use base64::{engine::general_purpose, Engine as _, DecodeError};
use std::str;
use std::panic;

#[derive(Debug)]
pub enum RuntimeError {
    Basic(),
    // Message(String),
    DecodeError(DecodeError),
    // From(aes_gcm::Error)
}

impl From<Box<dyn Any + Send>> for RuntimeError {
    fn from(_: Box<dyn Any + Send>) -> RuntimeError {
        return RuntimeError::Basic();
    }
}

impl From<aes_gcm::Error> for RuntimeError {
    fn from(_: aes_gcm::Error) -> RuntimeError {
        return RuntimeError::Basic();
    }
}

impl From<Utf8Error> for RuntimeError {
    fn from(_: Utf8Error) -> RuntimeError {
        return RuntimeError::Basic();
    }
}


pub fn gen_key(password: &String, salt: &[u8]) -> [u8; 32] {
    let rounds = 100_000; // Recommended iteration count for security
    let mut derived_key = [0_u8; 32]; // Buffer to store the 32-byte derived key (e.g., for AES-256)

    pbkdf2::<Hmac<Sha512>>(
        password.trim().as_bytes(),
        salt,
        rounds,
        &mut derived_key,
    )
    .expect("Error making key");
    return derived_key;
}

pub fn decode(s: &str) -> Result<Vec<u8>, base64::DecodeError> {
    return  general_purpose::STANDARD.decode(&s);
}

pub fn encode(b: &[u8]) -> String {
    return general_purpose::STANDARD.encode(b);
}


pub fn decrypt(to_decrpyt: &String, iv: &String, key: &[u8]) -> Result<String, RuntimeError> {

    // parse out tag and ciphertext
    let length = to_decrpyt.len();
    let val = &to_decrpyt[..length - 24];
    let tag = &to_decrpyt[length -24..];
    
    // parse to base64 and combine again
    let ciphertext = decode(&val)
        .map_err(RuntimeError::DecodeError)?;
    let tag = decode(&tag)
        .map_err(RuntimeError::DecodeError)?;
    let ciphertext_tag = [ciphertext, tag].concat();

    // this is nothing
    let aad = String::from("").into_bytes();

    // payload to encrypt
    let payload = Payload {
        msg: &ciphertext_tag[..],
        aad: &aad[..],
    };

    // create nonce from stored IV
    let x = decode(&iv)
        .map_err(RuntimeError::DecodeError)?;
    let nonce = panic::catch_unwind(|| {
        return Nonce::from_slice(&x[..]);
    })?;

    let key2 = Key::<Aes256Gcm>::from_slice(&key[..]);

    // do decrytpion
    let cipher = Aes256Gcm::new(&key2);
    let decrypted_vector = cipher.decrypt(&nonce, payload)?;

    // parse response to utf8
    let s = str::from_utf8(&decrypted_vector[..])?;
    return Ok(String::from(s));
}

pub fn encrypt(to_encrypt: &String, iv: &String, key: &[u8]) -> Result<String, RuntimeError> {

    // create key
    let key2 = Key::<Aes256Gcm>::from_slice(&key[..]);

    // create nonce from IV
    let nonce = general_purpose::STANDARD.decode(&iv).unwrap();
    let nonce = Nonce::from_slice(&nonce[..]);

    // this can be nothing
    let aad = String::from("").into_bytes();

    // make copy of data to encrypt since its done in place
    let mut cipherText = String::from(to_encrypt);
    unsafe {
        let x = cipherText.as_bytes_mut(); // unsafe turn into mut byte arr

        // encrypts
        let mut cipher = Aes256Gcm::new(&key2);
        let tag = cipher
            .encrypt_in_place_detached(nonce, &aad, x)?;

        // parse to base64 and combine data and tag
        let encrypted_b64 = general_purpose::STANDARD.encode(x);
        let tag_b64 = general_purpose::STANDARD.encode(tag);
        return Ok(encrypted_b64 + &tag_b64);
    }
    
}