pub fn generate_salt() -> String {
    use hex::encode;
    use rand::rngs::EntropyRng;
    use rand::RngCore;

    let mut buffer: [u8; 32] = [0; 32];
    EntropyRng::new().fill_bytes(&mut buffer);
    encode(buffer)
}

pub fn generate_session_token() -> String {
    use hex::encode;
    use rand::rngs::EntropyRng;
    use rand::RngCore;

    let mut buffer: [u8; 64] = [0; 64];
    EntropyRng::new().fill_bytes(&mut buffer);
    encode(&buffer[..])
}

pub fn hash_password(password: &String, salt: &String) -> String {
    use hex::encode;
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::default();
    hasher.input(format!("{}{}", password, salt).as_bytes());
    encode(hasher.result())
}
