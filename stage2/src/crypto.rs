use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key
};
use rand::RngCore;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

const KEY: &[u8; 32] = b"esta_es_una_clave_de_32_bytes___";

pub fn encrypt(plaintext: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key = Key::<Aes256Gcm>::from_slice(KEY);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Manejar el error manualmente sin usar ?
    let ciphertext = match cipher.encrypt(nonce, plaintext.as_bytes()) {
        Ok(c) => c,
        Err(e) => return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error cifrando: {}", e)
        ))),
    };

    let mut combined = Vec::with_capacity(12 + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    Ok(BASE64.encode(combined))
}

pub fn decrypt(ciphertext_b64: &str) -> Result<String, Box<dyn std::error::Error>> {
    let combined = match BASE64.decode(ciphertext_b64) {
        Ok(c) => c,
        Err(e) => return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error decodificando base64: {}", e)
        ))),
    };

    if combined.len() < 12 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Datos demasiado cortos"
        )));
    }

    let (nonce_bytes, ciphertext) = combined.split_at(12);

    let key = Key::<Aes256Gcm>::from_slice(KEY);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = match cipher.decrypt(nonce, ciphertext) {
        Ok(p) => p,
        Err(e) => return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error descifrando: {}", e)
        ))),
    };

    match String::from_utf8(plaintext) {
        Ok(s) => Ok(s),
        Err(e) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error convirtiendo a UTF-8: {}", e)
        ))),
    }
}
