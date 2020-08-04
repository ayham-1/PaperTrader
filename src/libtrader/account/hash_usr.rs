use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

/// Generates a client username hash from a raw username.
///
/// Takes in a raw username, outputs a hashed version of the client username to be sent to the server
/// with the returned client random bits that make up the whole client salt. This function is to be
/// used on client random bits that make up the whole client salt. This function is to be used on
/// client side account creation. The result from this function is not be stored directly on the
/// database, result must be run through the server side hashing again.
///
/// Arguments:
/// username - The raw user username to be hashed.
/// server_salt - The server's part sent of the salt.
///
/// Returns: a tuple containing the client hash and client's random salt, nothing on failure.
///
/// Example:
/// ```rust
///     let enc = hash_usr_client("n1ckn8me", server_salt).unwrap();
///     println!("Client Username Hash: {}", HEXUPPER.encode(&enc.0));
///     println!("Client Username Salt: {}", HEXUPPER.encode(&enc.1));
/// ```
pub fn hash_usr_client(usr: &str, server_salt: [u8; digest::SHA512_OUTPUT_LEN/2]) ->
Result<([u8; digest::SHA512_OUTPUT_LEN], [u8; digest::SHA512_OUTPUT_LEN/2]), ()> { // client hash, client random bits
    let client_iter: NonZeroU32 = NonZeroU32::new(100_000).unwrap();

    let rng = rand::SystemRandom::new();

    let mut client_salt = [0u8; digest::SHA512_OUTPUT_LEN/2];
    rng.fill(&mut client_salt).unwrap();

    let salt = [server_salt, client_salt].concat();

    let mut hash = [0u8; digest::SHA512_OUTPUT_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        client_iter,
        &salt,
        usr.as_bytes(),
        &mut hash);

    Ok((hash, client_salt))
}

/// Generates a storable server username hash from a client hashed username.
///
/// Takes in a client hashed username, outputs a storable new hash. The returned result is 'safe' to
/// be stored on the server side database. The salt returned is for the hashed version of the
/// hashed client username.
///
/// Arguments:
/// hashed_user- The client hashed username sent to the server.
///
/// Returns: a tuple containing the final hash and the hash's salt, nothing on failure.
///
/// Example:
/// ```rust
///     let enc = hash_usr_server("THISISTOTALLYAHASHEDTHING...").unwrap();
///     println!("Server Email Hash: {}", HEXUPPER.encode(&enc.0));
///     println!("Server Email Salt: {}", HEXUPPER.encode(&enc.1));
/// ```
pub fn hash_usr_server(hashed_usr: &str) ->
Result<([u8; digest::SHA512_OUTPUT_LEN], [u8; digest::SHA512_OUTPUT_LEN]), ()> {
    let client_iter: NonZeroU32 = NonZeroU32::new(100_000).unwrap();

    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; digest::SHA512_OUTPUT_LEN];
    rng.fill(&mut salt).unwrap();

    let mut hash = [0u8; digest::SHA512_OUTPUT_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        client_iter,
        &salt,
        hashed_usr.as_bytes(),
        &mut hash);

    Ok((hash, salt))
}

#[cfg(test)]
mod test {
    use super::*;
    use data_encoding::HEXUPPER;

    #[test]
    fn test_account_hash_usr_client() {
        let usr = "n1ckn8me";

        /* generate server salt */
        let rng = rand::SystemRandom::new();
        let mut server_salt = [0u8; digest::SHA512_OUTPUT_LEN/2];
        rng.fill(&mut server_salt).unwrap();

        /* ensure that hash_usr_client() works */
        match hash_usr_client(usr, server_salt) {
            Ok(output) => {
                assert_ne!(output.0.len(), 0);
                assert_ne!(output.1.len(), 0);
            },
            Err(()) => panic!("TEST_HASH_USERNAME_CLIENT_FAILED")
        }

        /* ensure that hash_usr_client() doesn't generate same output 
         * with the same server salt.
         * */
        let mut enc0 = hash_usr_client(usr, server_salt).unwrap();
        let mut enc1 = hash_usr_client(usr, server_salt).unwrap();
        assert_ne!(HEXUPPER.encode(&enc0.0), HEXUPPER.encode(&enc1.0));
        assert_ne!(HEXUPPER.encode(&enc0.1), HEXUPPER.encode(&enc1.1));

        /* ensure that hash_usr_client() generates a different output
         * with different server salts
         * */
        // Generate new server salt.
        let mut server_salt2 = [0u8; digest::SHA512_OUTPUT_LEN/2];
        rng.fill(&mut server_salt2).unwrap();

        enc0 = hash_usr_client(usr, server_salt).unwrap();
        enc1 = hash_usr_client(usr, server_salt).unwrap();
        assert_ne!(HEXUPPER.encode(&enc0.0), HEXUPPER.encode(&enc1.0));
        assert_ne!(HEXUPPER.encode(&enc0.1), HEXUPPER.encode(&enc1.1));
    }

    #[test]
    fn test_account_hash_usr_server() {
        let usr = "n1ckn8me";

        /* ensure that hash_usr_server() works */
        match hash_usr_server(usr) {
            Ok(output) => {
                assert_ne!(output.0.len(), 0);
                assert_ne!(output.1.len(), 0);
            },
            Err(()) => panic!("TEST_HASH_USERNAME_SERVER_FAILED")
        };

        /* ensure that hash_usr_server() generates different output
         * each time it is run.
         * */
        // Generate new server salt.
        let enc0 = hash_usr_server(usr).unwrap();
        let enc1 = hash_usr_server(usr).unwrap();
        assert_ne!(HEXUPPER.encode(&enc0.0), HEXUPPER.encode(&enc1.0));
        assert_ne!(HEXUPPER.encode(&enc0.1), HEXUPPER.encode(&enc1.1));

    }
}
