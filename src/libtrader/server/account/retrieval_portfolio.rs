use std::io::Write;

use crate::common::account::portfolio::Portfolio;
use crate::common::account::position::{Position, PositionType};
use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::message::message_builder::message_builder;

use crate::server::network::tls_connection::TlsConnection;
use crate::server::network::jwt_wrapper::verify_jwt_token;
use crate::server::db::config::{DB_PORTFOLIO_USER, DB_PORTFOLIO_PASS};
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

    /* connect to database */
    let mut client = db_connect(DB_PORTFOLIO_USER, DB_PORTFOLIO_PASS)?;

    /* get userId's portfolio open positions length */
    let mut pos_size = 0;
    for row in client.query(
        "SELECT array_length(open_positions, 1) FROM portfolio_schema.portfolios WHERE userid = $1",
        &[&token.user_id]).unwrap() {
        pos_size = row.get(0);
    }

    /* get userId's portfolio positions */
    let mut portfolio: Portfolio = Portfolio::default();
    // loop over the open positions
    while pos_size > 0 {
        // get open position table ID.
        let mut pos_id: i64 = 0;
        for row in client.query("SELECT open_positions[$1] FROM portfolio_schema.portfolios WHERE userid = $2",
                                &[&pos_size, &token.user_id]).unwrap() {
            pos_id = row.get(0);
        }
        // get position data from the portfolio_schema.positions table.
        for row in client.query("SELECT * FROM portfolio_schema.positions WHERE id = $1", &[&pos_id]).unwrap() {
            let mut pos: Position = Position::default();
            pos.stock_symbol = row.get(1);
            pos.stock_open_amount = row.get(2);
            pos.stock_open_price = row.get(3);
            pos.stock_open_cost = row.get(4);
            pos.stock_close_amount = row.get(5);
            pos.stock_close_price = row.get(6);
            pos.open_epoch = row.get(7);
            pos.is_open = row.get(9);
            
            let is_buy: bool = row.get(9);
            if  is_buy == false {
                pos.action_type = PositionType::Sell;
            } else {
                pos.action_type = PositionType::Buy;
            }

            portfolio.open_positions.push(pos);
        }

        // to the next position 
        pos_size = pos_size - 1;
    }

    /* build a message */
    match message_builder(MessageType::DataTransfer, 1, 1, 0, 0, bincode::serialize(&portfolio).unwrap()) {
        Ok(message) => {
            match tls_connection.tls_session.write(&bincode::serialize(&message).unwrap()) {
                Ok(_) => tls_connection.do_tls_write_and_handle_error(),
                Err(err) => return Err(format!("ACC_RETRIEVE_PORTFOLIO_FAILED_SENDING_MESSAGE: {}", err)),
            }
        },
        Err(_) => return Err("ACC_RETRIEVE_PORTFOLIO_MESSAGE_BUILD_FAILED".to_string())
    }

    Ok(())
}
