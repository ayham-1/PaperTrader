use std::sync::Arc;
use std::io;
use std::io::Read;

use mio;
use mio::net::TcpStream;
use rustls;
use rustls::Session;
use webpki;
use either::*;

use crate::network::handle_data::handle_data;

/// The TlsClient struct that represents a TLS client from the prespective of a client.
///
/// Members:
/// socket - The TcpStream for which TLS is used on.
/// closing - Used for startin a closing TlsClient state.
/// closed - Used for determining whether the TlsClient is closed.
/// tls_session - The ClientSession that is the TLS connection.
pub struct TlsClient {
    pub socket: TcpStream,
    pub closing: bool,
    pub clean_closure: bool,
    pub tls_session: rustls::ClientSession
}

impl TlsClient {
    /// Returns a new TlsClient struct.
    ///
    /// Arguments:
    /// sock - The ```TcpStream``` to be usd for the TlsClient.
    /// hostname - The hostname to connect to.
    /// cfg - The Client Configuration to be used.
    ///
    /// Returns: a new TlsClient
    pub fn new(sock: TcpStream, hostname: webpki::DNSNameRef<'_>, cfg: Arc<rustls::ClientConfig>) -> TlsClient {
        TlsClient {
            socket: sock,
            closing: false,
            clean_closure: false,
            tls_session: rustls::ClientSession::new(&cfg, hostname),
        }
    }

    /// TlsClient event reciever.
    ///
    /// Determines  if the mio::event::Event is readable/writable or is closing. Calls the
    /// appropriate TlsClient functin to handle the incoming event.
    /// 
    /// Arguments:
    /// ev - The event to be handled
    ///
    /// Returns: nothing
    pub fn ready(&mut self, ev: &mio::event::Event) {
        assert_eq!(ev.token(), mio::Token(0));

        if ev.is_readable() {
            self.do_read();
        }

        if ev.is_writable() {
            self.do_write();
        }

        if self.closing {
            warn!("TlsClient Closed");
        }
    }

    /// Private TlsClient function that reads incoming TlS packets.
    ///
    /// Reads TLS packets, decrypts them and then calls handle_data() on them.
    fn do_read(&mut self) {
        let rc = self.tls_session.read_tls(&mut self.socket);
        if rc.is_err() {
            let error = rc.unwrap_err();
            if error.kind() == io::ErrorKind::WouldBlock {
                return;
            }
            error!("TLS read error: {:?}", error);
            self.closing = true;
            return;
        }

        if rc.unwrap() == 0 {
            self.closing = true;
            self.clean_closure = true;
            return;
        }

        let processed = self.tls_session.process_new_packets();
        if processed.is_err() {
            error!("TLS error: {:?}", processed.unwrap_err());
            self.closing = true;
            return;
        }

        let mut plaintext = Vec::new();
        let rc = self.tls_session.read_to_end(&mut plaintext);
        if !plaintext.is_empty() {
            match handle_data(Either::Right(self), &plaintext) {
                Ok(()) => {},
                Err(err) => error!("Error handling data: {}", err)
            };
        }

        if rc.is_err() {
            let err = rc.unwrap_err();
            error!("tls plaintext read error: {:?}", err);
            self.clean_closure = err.kind() == io::ErrorKind::ConnectionAborted;
            self.closing = true;
        }
    }
    
    /// Private TlsClient function that writes buffered TLS packets.
    fn do_write(&mut self) {
        self.tls_session.write_tls(&mut self.socket).unwrap();
    }
    
    /// Registers the TlsClient to a mio::Registry
    ///
    /// Arguments:
    /// registry - The registry to register
    pub fn register(&mut self, registry: &mio::Registry) {
        let interest = self.ready_interest();
        registry.register(&mut self.socket, mio::Token(0), interest).unwrap();
    }

    pub fn reregister(&mut self, registry: &mio::Registry) {
        let interest = self.ready_interest();
        registry.reregister(&mut self.socket, mio::Token(0), interest).unwrap();
    }

    fn ready_interest(&self) -> mio::Interest {
        let rd = self.tls_session.wants_read();
        let wr = self.tls_session.wants_write();

        if rd && wr {
            mio::Interest::READABLE | mio::Interest::WRITABLE
        } else if wr {
            mio::Interest::WRITABLE
        } else {
            mio::Interest::READABLE
        }
    }
}

impl io::Write for TlsClient {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.tls_session.write(bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.tls_session.flush()
    }
}

impl io::Read for TlsClient {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        self.tls_session.read(bytes)
    }
}
