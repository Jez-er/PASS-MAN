use aes::Aes256;
use base64::engine::general_purpose::STANDARD;
use cipher::{BlockEncrypt, BlockDecrypt};
use cipher::{generic_array::GenericArray, BlockSizeUser, KeyInit};
use rand::Rng;
use base64::{engine::general_purpose, Engine as _};

const KEY_SIZE: usize = 32; // Key length (256 bits)
const IV_SIZE: usize = 16;  // Initialization vector length

// Random key generation for AES
pub fn generate_key() -> Vec<u8> {
    rand::thread_rng().gen::<[u8; KEY_SIZE]>().to_vec()
}

// Function for encoding in Base64
pub fn encode_key(key: &[u8]) -> String {
    general_purpose::STANDARD.encode(key)
}

// Function for decoding from Base64
pub fn decode_key(encoded_key: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(STANDARD.decode(encoded_key)?)
}

// Generating a random initialization vector (IV)
pub fn generate_iv() -> Vec<u8> {
    rand::thread_rng().gen::<[u8; IV_SIZE]>().to_vec()
}

pub fn encode_iv(iv: &[u8]) -> String {
    general_purpose::STANDARD.encode(iv)
}

pub fn decode_iv(encoded_iv: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(STANDARD.decode(encoded_iv)?)
}

/// Password encryption
pub fn encrypt_password(password: &str, key: &[u8], iv: &[u8]) -> Result<String, &'static str> {
    if key.len() != KEY_SIZE || iv.len() != IV_SIZE {
        return Err("ERROR | Invalid key or IV size");
    }

    let cipher = Aes256::new(GenericArray::from_slice(key));
    let buffer = password.as_bytes().to_vec();
    
    // Apply PKCS7 padding
    let mut padded = buffer.clone();
    let block_size = Aes256::block_size();
    let padding_needed = block_size - (buffer.len() % block_size);
    padded.extend(vec![padding_needed as u8; padding_needed]);

    // Encrypt the data
    cipher.encrypt_block(GenericArray::from_mut_slice(&mut padded));

    Ok(general_purpose::STANDARD.encode(&padded)) // Returning a Base64 representation
}

/// Дешифрование пароля
pub fn decrypt_password(encrypted: &str, key: &[u8], iv: &[u8]) -> Result<String, &'static str> {
    if key.len() != KEY_SIZE || iv.len() != IV_SIZE {
        return Err("ERROR | Invalid key or IV size");
    }

    let cipher = Aes256::new(GenericArray::from_slice(key));
    let mut encrypted_bytes = STANDARD.decode(encrypted).map_err(|_| "ERROR | Failed to decode Base64")?;

    // Decrypt the data
    cipher.decrypt_block(GenericArray::from_mut_slice(&mut encrypted_bytes));

    // Strip the padding (PKCS7)
    let padding = *encrypted_bytes.last().unwrap() as usize;
    encrypted_bytes.truncate(encrypted_bytes.len() - padding);

    // Convert the decrypted bytes into a string
    let decrypted_password = String::from_utf8(encrypted_bytes).map_err(|_| "ERROR | Failed to convert decrypted bytes to string")?;
    Ok(decrypted_password)
}
