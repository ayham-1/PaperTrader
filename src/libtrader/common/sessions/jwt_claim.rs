use serde::{Deserialize, Serialize};

/// JWT Claim representing an authorized user.
///
/// Members:
/// user_id - The user id that is authorized.
/// exp - The unix epoch at which this claim expires.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JWTClaim {
    pub user_id: i64,
    pub exp: u64,
}
