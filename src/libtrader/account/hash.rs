use ring::{digest, pbkdf2};
use std::num::NonZeroU32;

/// A generic hashing abstraction function.
/// 
/// Useful for quickly swapping the current hashing system.
///
/// Arguments:
/// val - The value to be hashed.
/// salt - The whole salt to be used.
/// iter - The number of iteration to use.
///
/// Returns: u8 array of size 64 bytes.
///
/// Example:
/// ```rust
///     let email_hash = hash("test@test.com", [0u8; 64], 124000);
/// ```
pub fn hash(val: &str, salt: Vec<u8>, iter: u32) -> [u8; digest::SHA512_OUTPUT_LEN] {
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
