use chrono::{DateTime, NaiveDateTime};
use std::next::Ipv4Addr;

static EXPIY_DATE: DateTime<Utc> = DateTime::<Utc>::from_utc(NaiveDateTime::from_ymd(0, 0, 30), Utc);

#[derive(PartialEq, Debug)]
pub struct SessionID  {
    pub sess_id: String,
    pub client_ip: Ipv4Addr,
    pub expiry_date: DateTime<Utc>,
    pub is_active: bool,
}
