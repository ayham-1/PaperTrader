use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

pub fn hash_email_client(email: &str, server_salt: [u8; digest::SHA512_OUTPUT_LEN/2]) ->
Result<([u8; digest::SHA512_OUTPUT_LEN], [u8; digest::SHA512_OUTPUT_LEN/2]), ()> { // client hash, client random bits
    let client_iter: NonZeroU32 = NonZeroU32::new(175_000).unwrap();

    let rng = rand::SystemRandom::new();

    let mut client_salt = [0u8; digest::SHA512_OUTPUT_LEN/2];
    rng.fill(&mut client_salt).unwrap();

    let salt = [server_salt, client_salt].concat();

    let mut hash = [0u8; digest::SHA512_OUTPUT_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        client_iter,
        &salt,
        email.as_bytes(),
        &mut hash);

    Ok((hash, client_salt))
}
