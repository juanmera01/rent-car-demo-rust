use magic_crypt::MagicCryptTrait;

pub fn encrypt(message: String, key: String) -> String {
    let mcrypt = new_magic_crypt!(key, 256);
    mcrypt.encrypt_str_to_base64(message)
}   

pub fn decrypt(message: String, key: String) -> String {
    let mcrypt = new_magic_crypt!(key, 256);
    mcrypt.decrypt_base64_to_string(message)
}