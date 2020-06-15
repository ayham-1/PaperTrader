use chrono::{DateTime, NaiveDateTime};
use std::next::Ipv4Addr;

static EXPIY_DATE: DateTime<Utc> = DateTime::<Utc>::from_utc(NaiveDateTime::from_ymd(0, 0, 30), Utc);

#[derive(PartialEq, Debug)]
pub struct SessionID  {
    sess_id: String,
    client_ip: Ipv4Addr,
    expiry_date: DateTime<Utc>,
    is_active: bool,
}
