use std::io::Write;

use crate::common::account::portfolio::Portfolio;
use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;

use crate::server::network::tls_connection::TlsConnection;
use crate::server::network::jwt_wrapper::verify_jwt_token;
use crate::server::db::config::{DB_ACC_USER, DB_ACC_PASS};
use crate::server::db::initializer::db_connect;

pub fn acc_retrieve_portfolio(tls_connection: &mut TlsConnection, message: &Message) -> Result<(), String> {
    /* verify JWT token */
    let token = match verify_jwt_token(bincode::deserialize(&message.data).unwrap()) {
        Ok(token) => token,
        Err(_) => {
            warn!("ACC_RETRIEVE_PORTFOLIO_UNAUTH_TOKEN");
            tls_connection.closing = true;
            return Err("ACC_REETRIEVE_PORTFOLIO_REJECTED".to_string());
        }
    };

    /* get userId's portfolio */
    let mut portfolio: Portfolio = Portfolio::default();

    Ok(())
}
