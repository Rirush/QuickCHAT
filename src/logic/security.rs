pub fn generate_salt() -> String {
    use rand::RngCore;
    use rand::rngs::EntropyRng;
    use hex::encode;

    let mut buffer: [u8; 32] = [0; 32];
    EntropyRng::new().fill_bytes(&mut buffer);
    encode(buffer)
}

pub fn hash_password(password: &String, salt: &String) -> String {
    use sha2::{Sha256,Digest};
    use hex::encode;
    use std::ops::Add;
    let mut hasher = Sha256::default();
    hasher.input(format!("{}{}", password, salt).as_bytes());
    encode(hasher.result())
}
