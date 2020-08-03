use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

pub fn hash_pwd_client(pass: &str) -> 
Result<([u8; digest::SHA512_OUTPUT_LEN], [u8;digest::SHA512_OUTPUT_LEN]), String> {
    let client_iter: NonZeroU32 = NonZeroU32::new(100_000).unwrap();

    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; digest::SHA512_OUTPUT_LEN];
    rng.fill(&mut salt).unwrap();

    let mut hash = [0u8; digest::SHA512_OUTPUT_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        client_iter,
        &salt,
        pass.as_bytes(),
        &mut hash);

    Ok((hash, salt))
}

pub fn hash_pwd_server(hashed_pass: &str) -> 
Result<([u8; digest::SHA512_OUTPUT_LEN], [u8;digest::SHA512_OUTPUT_LEN]), String> {
    let client_iter: NonZeroU32 = NonZeroU32::new(200_000).unwrap();
    
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; digest::SHA512_OUTPUT_LEN];
    rng.fill(&mut salt).unwrap();

    let mut hash = [0u8; digest::SHA512_OUTPUT_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        client_iter,
        &salt,
        hashed_pass.as_bytes(),
        &mut hash);

    Ok((hash, salt))
}
