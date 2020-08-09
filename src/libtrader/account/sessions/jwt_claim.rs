use serde::{Deserialize, Serialize};

#[cfg(not(feature="client"))]
pub static JWT_SECRET: &'static str = "seecreet";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JWTClaim {
    pub user_id: usize,
    pub exp: usize,
}
