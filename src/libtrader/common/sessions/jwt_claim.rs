use serde::{Deserialize, Serialize};

#[cfg(not(feature="client"))]
pub static JWT_SECRET: &'static str = "seecreet";

/// JWT Claim representing an authorized user.
///
/// Members:
/// user_id - The user id that is authorized.
/// exp - The unix epoch at which this claim expires.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JWTClaim {
    pub user_id: usize,
    pub exp: usize,
}
