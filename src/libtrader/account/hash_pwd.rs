use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

/// Generates a client hash from a raw password.
///
/// Takes in a raw password, outputs a hashed version of the client password to be sent to the
/// server with the returned client random bits that make up the whole client salt. This function
/// is to be used on client side account creation. The result from this function is not be stored
/// directly on the database, result must be run through the server side hashing again.
///
/// Arguments:
/// pass - The raw user password to be hashed.
/// server_salt - The server's part sent of the salt.
///
/// Returns: a tuple containing the client hash and client's random salt, nothing on failure.
///
/// Example:
/// ```rust
///     let enc = hash_pwd_client("this is my real password!", server_salt).unwrap();
///     println!("Client Hash: {}", HEXUPPER.encode(&enc.0));
///     println!("Client Salt: {}", HEXUPPER.encode(&enc.1));
/// ```
pub fn hash_pwd_client(pass: &str, server_salt: [u8; digest::SHA512_OUTPUT_LEN/2]) -> 
Result<([u8; digest::SHA512_OUTPUT_LEN], [u8; digest::SHA512_OUTPUT_LEN/2]), ()> { // client hash, client random bits
    let client_iter: NonZeroU32 = NonZeroU32::new(250_000).unwrap();

    let rng = rand::SystemRandom::new();

    let mut client_salt = [0u8; digest::SHA512_OUTPUT_LEN/2];
    rng.fill(&mut client_salt).unwrap();

    let salt = [server_salt, client_salt].concat();

    let mut hash = [0u8; digest::SHA512_OUTPUT_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        client_iter,
        &salt,
        pass.as_bytes(),
        &mut hash);

    Ok((hash, client_salt))
}

/// Generates a storable server hash from a client hashed password.
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
///     let enc = hash_pwd_server("THISISTOTALLYAHASHEDTHING...").unwrap();
///     println!("Server Hash: {}", HEXUPPER.encode(&enc.0));
///     println!("Server Salt: {}", HEXUPPER.encode(&enc.1));
/// ```
pub fn hash_pwd_server(hashed_pass: &str) -> 
Result<([u8; digest::SHA512_OUTPUT_LEN], [u8;digest::SHA512_OUTPUT_LEN]), ()> {
    let client_iter: NonZeroU32 = NonZeroU32::new(500_000).unwrap();
    
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

#[cfg(test)]
mod test {
    use super::*;
    use data_encoding::HEXUPPER; 

    #[test]
    fn test_account_hash_pwd_client() {
        let pass = "goodlilpassword";

        /* generate server salt */
        let rng = rand::SystemRandom::new();
        let mut server_salt = [0u8; digest::SHA512_OUTPUT_LEN/2];
        rng.fill(&mut server_salt).unwrap();

        /* ensure that hash_pwd_client() works */
        match hash_pwd_client(pass, server_salt) {
            Ok(output) => {
                assert_ne!(output.0.len(), 0);
                assert_ne!(output.1.len(), 0);
            },
            Err(()) => panic!("TEST_HASH_PWD_CLIENT_FAILED")
        };

        /* ensure that hash_pwd_client() doesn't generate same output 
         * with the same server salt.
         * */
        let mut enc0 = hash_pwd_client(pass, server_salt).unwrap();
        let mut enc1 = hash_pwd_client(pass, server_salt).unwrap();
        assert_ne!(HEXUPPER.encode(&enc0.0), HEXUPPER.encode(&enc1.0));
        assert_ne!(HEXUPPER.encode(&enc0.1), HEXUPPER.encode(&enc1.1));

        /* ensure that hash_pwd_client() generates different output
         * with different server salts.
         * */
        // Generate new server salt.
        let mut server_salt2 = [0u8; digest::SHA512_OUTPUT_LEN/2];
        rng.fill(&mut server_salt2).unwrap();

        enc0 = hash_pwd_client(pass, server_salt).unwrap();
        enc1 = hash_pwd_client(pass, server_salt).unwrap();
        assert_ne!(HEXUPPER.encode(&enc0.0), HEXUPPER.encode(&enc1.0));
        assert_ne!(HEXUPPER.encode(&enc0.1), HEXUPPER.encode(&enc1.1));
    }

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
