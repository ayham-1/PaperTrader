use crate::account::sessions::jwt_claim::JWTClaim;

#[cfg(not(feature="client"))]
pub fn create_jwt_token(user_id: usize, exp: usize) -> Result<String, String> {
    use crate::account::sessions::jwt_claim::JWT_SECRET;
    use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};

    let mut header = Header::default();
    header.alg = Algorithm::HS512;

    let claim = JWTClaim {
        user_id: user_id,
        issuer: "ISSUE-R".to_string(),
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

    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 25;
    validation.iss = Some("ISSUE-R".to_string());
    match decode::<JWTClaim>(&token, &DecodingKey::from_secret(JWT_SECRET.to_string().as_bytes()), 
                                              &validation) {
        Ok(data) => Ok(data.claims),
        Err(_) => Err(())
    }
}
