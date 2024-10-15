use crate::{cvec, KEY_};
use aes_gcm::aead::consts::U12;
use aes_gcm::aead::{Aead, Nonce};
use aes_gcm::aes::Aes256;
use aes_gcm::{AeadCore, Aes256Gcm, AesGcm, KeyInit};
use rand_core::OsRng;

pub fn encrypt(data: &[u8]) -> Result<(cvec::CVec<u8>, Nonce<AesGcm<Aes256, U12>>), String> {
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(*KEY_);

    match cipher.encrypt(&nonce, data.as_ref()) {
        Ok(ciphertext) => Ok((cvec::CVec(ciphertext), nonce)),
        Err(_) => Err("Encryption failure!".to_string())
    }
}
pub fn decrypt(encrypted_data: &[u8], nonce: &Nonce::<AesGcm<Aes256, U12>>) -> cvec::CVec<u8> {
    let cipher = Aes256Gcm::new(*KEY_);
    let decrypted_data = cipher.decrypt(&nonce, encrypted_data.as_ref()).expect("decryption failure!");
    cvec::CVec(decrypted_data)
}