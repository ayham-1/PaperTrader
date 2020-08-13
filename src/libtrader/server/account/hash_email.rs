use ring::rand::SecureRandom;
use ring::{digest, rand};

use crate::common::account::hash::hash;

/// Generates a storable server email hash from a client hashed email.
///
/// Takes in a client hashed email, outputs a storable new hash. The returned result is 'safe' to
/// be stored on the server side database. The salt returned is for the hashed version of the
/// hashed client email.
///
/// Arguments:
/// hashed_email - The client hashed email sent to the server.
///
/// Returns: a tuple containing the final hash and the hash's salt, nothing on failure.
///
/// Example:
/// ```rust
///     let enc = hash_email("THISISTOTALLYAHASHEDTHING...").unwrap();
///     println!("Server Email Hash: {}", HEXUPPER.encode(&enc.0));
///     println!("Server Email Salt: {}", HEXUPPER.encode(&enc.1));
/// ```
pub fn hash_email(hashed_email: &Vec<u8>) ->
Result<([u8; digest::SHA512_OUTPUT_LEN], [u8; digest::SHA512_OUTPUT_LEN]), ()> {
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; digest::SHA512_OUTPUT_LEN];
    rng.fill(&mut salt).unwrap();

    let hash = hash(hashed_email, &salt.to_vec(), 350_000);
    Ok((hash, salt))
}

#[cfg(test)]
mod test {
    use super::*;
    use data_encoding::HEXUPPER;

    #[test]
    fn test_account_hash_email_server() {
        let email = "totallyrealemail@anemail.c0m";

        /* ensure that hash_email_server() works */
        match hash_email_server(email) {
            Ok(output) => {
                assert_ne!(output.0.len(), 0);
                assert_ne!(output.1.len(), 0);
            },
            Err(()) => panic!("TEST_HASH_EMAIL_SERVER_FAILED")
        };

        /* ensure that hash_email_server() generates different output
         * each time it is run.
         * */
        // Generate new server salt.
        let enc0 = hash_email_server(email).unwrap();
        let enc1 = hash_email_server(email).unwrap();
        assert_ne!(HEXUPPER.encode(&enc0.0), HEXUPPER.encode(&enc1.0));
        assert_ne!(HEXUPPER.encode(&enc0.1), HEXUPPER.encode(&enc1.1));

    }
}
