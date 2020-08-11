use ring::rand::SecureRandom;
use ring::{digest, rand};

use crate::account::hash::hash;

/// Generates a storable server password hash from a client hashed password.
/// 
/// Takes in a client hashed password, outputs a storable new hash. The returned result is 'safe'
/// to be stored on the server side. The salt returned is for the hashed version of the hashed
/// client password.
///
/// Arguments:
/// hashed_pass - The client hashed password sent to the server. 
///
/// Returns: a tuple containing the final hash and the hash's salt, nothing on failure.
///
/// Example:
/// ```rust
///     let enc = hash_pwd("THISISTOTALLYAHASHEDTHING...").unwrap();
///     println!("Server Hash: {}", HEXUPPER.encode(&enc.0));
///     println!("Server Salt: {}", HEXUPPER.encode(&enc.1));
/// ```
#[cfg(feature="server")]
pub fn hash_pwd(hashed_pass: &str) -> 
Result<([u8; digest::SHA512_OUTPUT_LEN], [u8; digest::SHA512_OUTPUT_LEN]), ()> { // sever hash, server salt
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; digest::SHA512_OUTPUT_LEN];
    rng.fill(&mut salt).unwrap();

    let hash = hash(hashed_pass, salt.to_vec(), 500_000);
    Ok((hash, salt))
}

#[cfg(test)]
mod test {
    use super::*;
    use data_encoding::HEXUPPER; 

    #[test]
    fn test_account_hash_pwd_server() {
        let pass = "goodlilpassword";

        /* ensure that hash_pwd_server() works */
        match hash_pwd_server(pass) {
            Ok(output) => {
                assert_ne!(output.0.len(), 0);
                assert_ne!(output.1.len(), 0);
            },
            Err(()) => panic!("TEST_HASH_PWD_SERVER_FAILED")
        };

        /* ensure that hash_pwd_server() generates different output
         * each time it is run.
         * */
        // Generate new server salt.
        let enc0 = hash_pwd_server(pass).unwrap();
        let enc1 = hash_pwd_server(pass).unwrap();
        assert_ne!(HEXUPPER.encode(&enc0.0), HEXUPPER.encode(&enc1.0));
        assert_ne!(HEXUPPER.encode(&enc0.1), HEXUPPER.encode(&enc1.1));
    }
}
