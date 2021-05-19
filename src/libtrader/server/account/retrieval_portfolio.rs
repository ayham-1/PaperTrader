use std::io::Write;

use crate::common::account::portfolio::Portfolio;
use crate::common::account::position::Position;
use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;
use crate::common::message::message_type::MessageType;
use crate::common::misc::return_flags::ReturnFlags;

use crate::server::db::config::{DB_PORTFOLIO_PASS, DB_PORTFOLIO_USER};
use crate::server::db::initializer::db_connect;
use crate::server::network::jwt_wrapper::verify_jwt_token;
use crate::server::network::tls_connection::TlsConnection;

pub fn acc_retrieve_portfolio(
    tls_connection: &mut TlsConnection,
    message: &Message,
) -> Result<(), ReturnFlags> {
    /* verify JWT token */
    let token = match verify_jwt_token(bincode::deserialize(&message.data).unwrap()) {
        Ok(token) => token,
        Err(_) => {
            warn!("ACC_RETRIEVE_PORTFOLIO_UNAUTH_TOKEN");
            tls_connection.closing = true;
            return Err(ReturnFlags::SERVER_ACC_UNAUTHORIZED);
        }
    };

    /* connect to database */
    let mut client = db_connect(DB_PORTFOLIO_USER, DB_PORTFOLIO_PASS)?;

    /* get userId's portfolio positions */
    let mut portfolio: Portfolio = Portfolio::default();
    // get position data from the portfolio_schema.positions table.
    for row in client
        .query(
            "SELECT * FROM portfolio_schema.positions WHERE user_id = $1",
            &[&token.user_id],
        )
        .unwrap()
    {
        let mut pos: Position = Position::default();
        pos.stock_symbol = row.get(2);
        pos.stock_open_amount = row.get(3);
        pos.stock_open_price = row.get(4);
        pos.stock_open_cost = row.get(5);
        pos.stock_close_amount = row.get(6);
        pos.stock_close_price = row.get(7);
        pos.open_epoch = row.get(8);
        pos.close_epoch = row.get(9);
        pos.is_open = row.get(10);

        pos.is_buy = row.get(11);
        portfolio.open_positions.push(pos);
    }

    /* build a message */
    let message = message_builder(
        MessageType::DataTransfer,
        1,
        1,
        0,
        0,
        bincode::serialize(&portfolio).unwrap(),
    );
    let _ = tls_connection.write(&bincode::serialize(&message).unwrap());

    Ok(())
}
