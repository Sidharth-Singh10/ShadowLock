use argon2::{Argon2, password_hash::{PasswordHasher, SaltString}};
use base64::{Engine, prelude::BASE64_STANDARD};
use std::error::Error;

pub fn generate_password(master_password: &str, site_name: &str, pre_password: &str) -> Result<String, Box<dyn Error>> {
    let salt_str = std::env::var("PASSWORD_SALT").expect("PASSWORD_SALT must be set in .env file");
    
    let argon2 = Argon2::default();
    let salt = SaltString::from_b64(BASE64_STANDARD.encode(&salt_str).as_str())
        .expect("Invalid salt");
    
    let password_hash = argon2
        .hash_password(master_password.as_bytes(), &salt)
        .expect("Password hashing failed");
    
    // Extract the hash part to use as our private key
    let hash_string = password_hash.hash.expect("Hash value missing").to_string();
    
    // Create a BLAKE3 hash of the Argon2 hash to get a 32-byte key
    let keyed_hash = blake3::hash(hash_string.as_bytes());
    let private_key_array = keyed_hash.as_bytes();
    
    let mut blake_hasher = blake3::Hasher::new_keyed(private_key_array);
    
    // Add site name
    blake_hasher.update(site_name.as_bytes());
    
    // Add pre_password
    blake_hasher.update(pre_password.as_bytes());
    
    let real_password = blake_hasher.finalize().as_bytes().to_vec();
    
    let real_password = real_password
        .iter()
        .take(15)
        .map(|&byte| (byte % 94 + 33) as char) // Map bytes to printable ASCII range
        .collect::<String>();
    
    Ok(real_password)
}