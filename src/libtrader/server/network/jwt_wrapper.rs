use crate::common::misc::return_flags::ReturnFlags;
use crate::common::sessions::jwt_claim::JWTClaim;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

pub static JWT_SECRET: &'static str = "seecreet";

/// Encodes a JWT token.
///
/// Takes in the authorized user id and expiry date of the authorization.
/// Outputs a string of the token.
///
/// Arguments:
/// user_id - The DB entry id of the authorized user.
/// exp - The unix epoch at which the token expires
///
/// Returns: a string of the token on success, string on error.
///
/// Example:
/// ```rust
///     let token = create_jwt_token(auth_user_id, unix_expiry_epoch).unwrap();
/// ```
pub fn create_jwt_token(user_id: i64, exp: u64) -> Result<String, ReturnFlags> {
    let mut header = Header::default();
    header.alg = Algorithm::HS512;

    let claim = JWTClaim {
        user_id: user_id,
        exp: exp,
    };
    match encode(
        &header,
        &claim,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    ) {
        Ok(token) => Ok(token),
        Err(_) => Err(ReturnFlags::ServerCreateJwtTokenFailed),
    }
}

/// Decodes and verifies a JWT token.
///
/// Takes in an encoded JWT token and outputs it's JWTClaim.
///
/// Arguments:
/// token - the JWT token to be decoded
///
/// Returns: JWTClaim on success, nothing on error.
///
/// Example:
/// ```rust
///     assert_eq!(verify_jwt_token(token).unwrap(), true);
/// ```
pub fn verify_jwt_token(token: String) -> Result<JWTClaim, ()> {
    let mut validation = Validation::new(Algorithm::HS512);
    validation.leeway = 25;
    match decode::<JWTClaim>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &validation,
    ) {
        Ok(data) => Ok(data.claims),
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_jwt_token() {
        use std::time::{Duration, SystemTime, UNIX_EPOCH};
        let start = SystemTime::now() + Duration::from_secs(4 * 60 * 60);
        match create_jwt_token(1i64, start.duration_since(UNIX_EPOCH).unwrap().as_secs()) {
            Ok(token) => {
                let claims = verify_jwt_token(token).unwrap();
                assert_eq!(claims.user_id, 1i64);
                assert_eq!(
                    claims.exp,
                    start.duration_since(UNIX_EPOCH).unwrap().as_secs()
                );
            }
            Err(_) => panic!("TEST_CREATE_JWT_TOKEN_FAILED"),
        }
    }

    #[test]
    fn test_verify_jwt_token() {
        use std::time::{Duration, SystemTime, UNIX_EPOCH};
        let start = SystemTime::now() + Duration::from_secs(4 * 60 * 60);
        let token =
            create_jwt_token(1i64, start.duration_since(UNIX_EPOCH).unwrap().as_secs()).unwrap();
        match verify_jwt_token(token) {
            Ok(claims) => {
                assert_eq!(claims.user_id, 1i64);
                assert_eq!(
                    claims.exp,
                    start.duration_since(UNIX_EPOCH).unwrap().as_secs()
                );
            }
            Err(_) => panic!("TEST_VERIFY_JWT_TOKEN_FAILED"),
        }
    }
}
