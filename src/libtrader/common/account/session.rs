use chrono::{DateTime, Utc};
use std::net::Ipv4Addr;

#[derive(PartialEq, Debug)]
pub struct SessionID {
    pub sess_id: String,
    pub client_ip: Ipv4Addr,
    pub expiry_date: DateTime<Utc>,
    pub is_active: bool,
}
impl std::fmt::Display for SessionID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.sess_id, self.client_ip, self.expiry_date, self.is_active
        )
    }
}
