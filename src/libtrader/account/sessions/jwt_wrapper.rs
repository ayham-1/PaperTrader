use crate::account::sessions::jwt_claim::JWTClaim;

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
#[cfg(not(feature="client"))]
pub fn create_jwt_token(user_id: usize, exp: usize) -> Result<String, String> {
    use crate::account::sessions::jwt_claim::JWT_SECRET;
    use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};

    let mut header = Header::default();
    header.alg = Algorithm::HS512;

    let claim = JWTClaim {
        user_id: user_id,
        exp: exp
    };
    match encode(&header, &claim, &EncodingKey::from_secret(JWT_SECRET.as_bytes())) {
        Ok(token) => Ok(token),
        Err(_) => Err("CREATE_JWT_TOKEN_FAILED".to_string())
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
#[cfg(not(feature="client"))]
pub fn verify_jwt_token(token: String) -> Result<JWTClaim, ()> {
    use crate::account::sessions::jwt_claim::JWT_SECRET;
    use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

    let mut validation = Validation::new(Algorithm::HS512);
    validation.leeway = 25;
    match decode::<JWTClaim>(&token, &DecodingKey::from_secret(JWT_SECRET.as_bytes()), 
                                              &validation) {
        Ok(data) => { Ok(data.claims)},
        Err(_) => Err(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_jwt_token() {
        use std::time::{SystemTime, UNIX_EPOCH, Duration};
        let start = SystemTime::now() + Duration::from_secs(4*60*60);
        match create_jwt_token(1usize, start.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize) {
            Ok(token) => {
                let claims = verify_jwt_token(token).unwrap();
                assert_eq!(claims.user_id, 1usize);
                assert_eq!(claims.exp, start.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize);
            },
            Err(_) => panic!("TEST_CREATE_JWT_TOKEN_FAILED")
        }
    }

    #[test]
    fn test_verify_jwt_token() {
        use std::time::{SystemTime, UNIX_EPOCH, Duration};
        let start = SystemTime::now() + Duration::from_secs(4*60*60);
        let token = create_jwt_token(1usize, start.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize).unwrap();
        match verify_jwt_token(token) {
            Ok(claims) => {
                assert_eq!(claims.user_id, 1usize);
                assert_eq!(claims.exp, start.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize);
            },
            Err(_) => panic!("TEST_VERIFY_JWT_TOKEN_FAILED")
        }
    }
}
