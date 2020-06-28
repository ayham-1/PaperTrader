use chrono::{DateTime, Utc};
use std::net::Ipv4Addr;

#[derive(PartialEq, Debug)]
pub struct SessionID  {
    pub sess_id: String,
    pub client_ip: Ipv4Addr,
    pub expiry_date: DateTime<Utc>,
    pub is_active: bool,
}
