use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

/// Generates a client email hash from a raw email.
///
/// Takes in a raw email, outputs a hashed version of the client email to be sent to the server
/// with the returned client random bits that make up the whole client salt. This function is to be
/// used on client random bits that make up the whole client salt. This function is to be used on
/// client  side account creation. The result from this function is not be stored directly on the
/// database, result must be run through the server side hashing again.
///
/// Arguments:
/// email - The raw user email to be hashed.
/// server_salt - The server's part sent of the salt.
///
/// Returns: a tuple containing the client hash and client's random salt, nothing on failure.
///
/// Example:
/// ```rust
///     let enc = hash_email_client("totallyrealemail@anemail.c0m", server_salt).unwrap();
///     println!("Client Email Hash: {}", HEXUPPER.encode(&enc.0));
///     println!("Client Email Salt: {}", HEXUPPER.encode(&enc.1));
/// ```
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
///     let enc = hash_email_server("THISISTOTALLYAHASHEDTHING...").unwrap();
///     println!("Server Email Hash: {}", HEXUPPER.encode(&enc.0));
///     println!("Server Email Salt: {}", HEXUPPER.encode(&enc.1));
/// ```
pub fn hash_email_server(hashed_email: &str) ->
Result<([u8; digest::SHA512_OUTPUT_LEN], [u8; digest::SHA512_OUTPUT_LEN]), ()> {
    let client_iter: NonZeroU32 = NonZeroU32::new(350_000).unwrap();

    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; digest::SHA512_OUTPUT_LEN];
    rng.fill(&mut salt).unwrap();

    let mut hash = [0u8; digest::SHA512_OUTPUT_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        client_iter,
        &salt,
        hashed_email.as_bytes(),
        &mut hash);

    Ok((hash, salt))
}

#[cfg(test)]
mod test {
    use super::*;
    use data_encoding::HEXUPPER;

    #[test]
    fn test_account_hash_email_client() {
        let email = "totallyrealemail@anemail.c0m";

        /* generate server salt */
        let rng = rand::SystemRandom::new();
        let mut server_salt = [0u8; digest::SHA512_OUTPUT_LEN/2];
        rng.fill(&mut server_salt).unwrap();

        /* ensure that hash_email_client() works */
        match hash_email_client(email, server_salt) {
            Ok(output) => {
                assert_ne!(output.0.len(), 0);
                assert_ne!(output.1.len(), 0);
            },
            Err(()) => panic!("TEST_HASH_EMAIL_CLIENT_FAILED")
        }

        /* ensure that hash_email_client() doesn't generate same output 
         * with the same server salt.
         * */
        let mut enc0 = hash_email_client(email, server_salt).unwrap();
        let mut enc1 = hash_email_client(email, server_salt).unwrap();
        assert_ne!(HEXUPPER.encode(&enc0.0), HEXUPPER.encode(&enc1.0));
        assert_ne!(HEXUPPER.encode(&enc0.1), HEXUPPER.encode(&enc1.1));

        /* ensure that hash_email_client() generates a different output
         * with different server salts
         * */
        // Generate new server salt.
        let mut server_salt2 = [0u8; digest::SHA512_OUTPUT_LEN/2];
        rng.fill(&mut server_salt2).unwrap();

        enc0 = hash_email_client(email, server_salt).unwrap();
        enc1 = hash_email_client(email, server_salt).unwrap();
        assert_ne!(HEXUPPER.encode(&enc0.0), HEXUPPER.encode(&enc1.0));
        assert_ne!(HEXUPPER.encode(&enc0.1), HEXUPPER.encode(&enc1.1));
    }
}
