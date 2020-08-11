use crate::common::message::message::Message;
use crate::common::message::inst::CommandInst;

use crate::server::network::tls_connection::TlsConnection;
use crate::server::network::cmd::register::register;

pub fn handle_data(connection: &mut TlsConnection, buf: &[u8]) -> 
Result<(), String> {
    /* decode incoming message */
    let client_response: Message = match bincode::deserialize(&buf) {
        Ok(msg) => msg,
        Err(err) => {
            warn!("HANDLE_DATA_RCVD_INV_MSG: {}", err); 
            connection.closing = true; /* disconnect any unrecognized message senders */
            return Ok(());
        }
    };

    /* handle individual client instructions */
    match client_response.instruction {
        _ if client_response.instruction == CommandInst::Register as i64 => 
            register(connection, &client_response),
        _ => {}
    };
        
    Ok(())
}
