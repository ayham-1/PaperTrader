use ring::{digest, pbkdf2};
use std::num::NonZeroU32;

pub fn hash(val: &str, salt: [u8; digest::SHA512_OUTPUT_LEN], iter: u32) -> [u8; digest::SHA512_OUTPUT_LEN] {
    let iterations: NonZeroU32 = NonZeroU32::new(iter).unwrap();

    let mut hash = [0u8; digest::SHA512_OUTPUT_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        iterations,
        &salt,
        val.as_bytes(),
        &mut hash);
    hash
}
