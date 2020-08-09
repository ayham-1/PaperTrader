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
