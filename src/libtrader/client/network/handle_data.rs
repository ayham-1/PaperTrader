use either::*;

use crate::network::tls_connection::TlsConnection;
use crate::network::tls_client::TlsClient;

pub fn handle_data(conn: Either<&mut TlsConnection, &mut TlsClient>, _buf: &[u8]) -> Result<(), String> {
    assert_eq!(conn.is_right(), true);
    Ok(())
}
