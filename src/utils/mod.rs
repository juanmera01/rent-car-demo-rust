use magic_crypt::MagicCryptTrait;
use magic_crypt::new_magic_crypt;

pub fn encrypt(message: String, key: &str) -> String {
    let mcrypt = new_magic_crypt!(key, 256);
    mcrypt.encrypt_str_to_base64(message)
}   

pub fn decrypt(message: String, key: &str) -> String {
    let mcrypt = new_magic_crypt!(key, 256);
    mcrypt.decrypt_base64_to_string(message).unwrap()
}