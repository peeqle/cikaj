use crate::{cvec, KEY_};
use aes_gcm::aead::consts::U12;
use aes_gcm::aead::{Aead, Nonce};
use aes_gcm::aes::Aes256;
use aes_gcm::{AeadCore, Aes256Gcm, AesGcm, KeyInit};
use rand_core::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey};

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

pub fn generate_key() {
    let client_secret = EphemeralSecret::random_from_rng(OsRng);
    let client_public = PublicKey::from(&client_secret);

    let server_secret = EphemeralSecret::diffie_hellman(OsRng);
    let server_public = PublicKey::from(&server_secret);

    // Клиент генерирует общий ключ
    let client_shared_secret = client_secret.diffie_hellman(&server_public);
    println!("Client shared secret: {:?}", client_shared_secret.as_bytes());

    // Сервер генерирует общий ключ
    let server_shared_secret = server_secret.diffie_hellman(&client_public);
    println!("Server shared secret: {:?}", server_shared_secret.as_bytes());

    // Оба ключа должны совпадать
    assert_eq!(client_shared_secret.as_bytes(), server_shared_secret.as_bytes());
}

