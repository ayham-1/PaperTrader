use std::sync::Arc;
use std::collections::HashMap;
use std::io;

use mio;
use mio::net::TcpListener;
use rustls;

use crate::network::tls_connection::TlsConnection;

/// The TlsServer struct that represents a working TLS Server
///
/// Members:
/// server - The TcpListener for which TLS is accepted.
/// connections - The map of all connections.
/// next_id - The next unused ID for registering in the mio::Poll
/// tls_config - The TLS server configuration.
#[derive(Debug)]
pub struct TlsServer {
    pub server: TcpListener,
    pub connections: HashMap<mio::Token, TlsConnection>,
    pub next_id: usize,
    pub tls_config: Arc<rustls::ServerConfig>,
}

impl TlsServer {
    /// Returns a new TlsServer struct
    ///
    /// Arguments:
    /// server - The ```TcpListener``` to be used for the TLS Server.
    /// cfg - The TLS server configuration to be used.
    ///
    /// Returns: a new TlsServer
    pub fn new(server: TcpListener, cfg: Arc<rustls::ServerConfig>) -> TlsServer {
        TlsServer {
            server,
            connections: HashMap::new(),
            next_id: 1,
            tls_config: cfg,
        }
    }

    /// Accepts any new TLS connections.
    ///
    /// Arguments:
    /// registry - The registry to register connections.
    ///
    /// Returns: nothing on success, io::Error on failure.
    pub fn accept(&mut self, registry: &mio::Registry) -> Result<(), io::Error> {
        loop {
            match self.server.accept() {
                Ok((socket, addr)) => {
                    debug!("Accepting new connection from {:?}", addr);

                    let tls_session = rustls::ServerSession::new(&self.tls_config);

                    let token = mio::Token(self.next_id);
                    self.next_id += 1;

                    let mut connection = TlsConnection::new(socket, token, tls_session);
                    connection.register(registry);
                    self.connections.insert(token, connection);
                },
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => return Ok(()),
                Err(err) => {
                    error!("tls server error accepting connections; err={:?}", err);
                    return Err(err);
                }
            }
        }
    }

    pub fn conn_event(&mut self, registry: &mio::Registry, event: &mio::event::Event) {
        let token = event.token();

        if self.connections.contains_key(&token) {
            self.connections.get_mut(&token).unwrap().ready(registry, event);

            if self.connections[&token].closed {
                self.connections.remove(&token);
            }
        }
    }
}
