use std::io;
use std::net;
use std::io::{Read};

use mio;
use mio::net::TcpStream;
use rustls;
use rustls::Session;

use crate::network::handle_data::handle_data;

#[derive(Debug)]
pub struct TlsConnection {
    pub socket: TcpStream,
    pub token: mio::Token,
    pub closing: bool,
    pub closed: bool,
    pub tls_session: rustls::ServerSession,
}

impl TlsConnection {
    /// Returns a new TlsConnection struct.
    ///
    /// Arguments:
    /// socket - The TcpStream to be used for the TlsConnection
    /// token - The mio::Token to be used to identify the TlsConnection
    /// tls_session - The rustls::ServerSession to be used in the Tls Connection
    ///
    /// Returns: a new TlsConnection
    pub fn new(socket: TcpStream,
           token: mio::Token,
           tls_session: rustls::ServerSession)
        -> TlsConnection {
            TlsConnection {
                socket,
                token,
                closing: false,
                closed: false,
                tls_session,
            }
        }

    /// TlsConnection event reciever.
    ///
    /// Determines if the mio::event::Event is readable/writable or is closing. Calls the
    /// appropriate TlsConnection function to handle the event. The event is not sent to the
    /// receiving function.
    ///
    /// Arguments:
    /// registry - The mio::Registry containing the mio::Tokens.
    /// event - The event to be 'dispatched'.
    ///
    /// Returns: nothing
    pub fn ready(&mut self, registry: &mio::Registry, ev: &mio::event::Event) {
        if ev.is_readable() {
            self.do_tls_read();
            self.try_plain_read();
        }

        if ev.is_writable() {
            self.do_tls_write_and_handle_error();
        }

        if self.closing {
            let _ = self.socket.shutdown(net::Shutdown::Both);
            self.closed = true;
            self.deregister(registry);
        } else {
            self.reregister(registry);
        }
    }

    /// Private TlsConnection function that reads incoming TLS packets.
    ///
    /// Reads the ```self.tls_session``` received TLS packets and then handles any errors
    /// generated. If reading TLS packets succeeds, issue new packets to be ready for reading.
    fn do_tls_read(&mut self) {
        // read some tls data.
        let rc = self.tls_session.read_tls(&mut self.socket);
        if rc.is_err() {
            let err = rc.unwrap_err();

            if let io::ErrorKind::WouldBlock = err.kind() { /* make this simpler */
                return;
            }

            error!("tls read error: {:?}", err);
            self.closing = true;
            return;
        }
        
        if rc.unwrap() == 0 {
            self.closing = true;
            return;
        }

        // process newly-recieved tls messages.
        let processed = self.tls_session.process_new_packets();
        if processed.is_err() {
            error!("tls cannot process packet: {:?}", processed);

            // last gasp write to send any alerts
            self.do_tls_write_and_handle_error();

            self.closing = true;
            return;
        }
    }

    /// Private TlsConnection function that reads decrypted TLS packets.
    ///
    /// Reads from ``self.tls_sessions``` and fills a buffer. The buffer is sent to
    /// ```self.incoming_plaintext()```.
    fn try_plain_read(&mut self) {
        // read and process all available plaintext.
        let mut buf = Vec::new();

        let rc = self.tls_session.read_to_end(&mut buf);
        if rc.is_err() {
            error!("tls plaintext read failed: {:?}", rc);
            self.closing = true;
            return;
        }

        if !buf.is_empty() {
            debug!("plaintext read {:?}", buf.len());
            self.incoming_plaintext(&buf);
        }
    }

    /// Private TlsConnection function that dispatches decrypted TLS packets.
    ///
    /// Dispatches decrypted TLS data to ```handle_data()```.
    fn incoming_plaintext(&mut self, buf: &[u8]) {
        match handle_data(self, buf) {
            Ok(()) => {},
            Err(err) => error!("Error processing TLS connection: {}", err)
        }
    }

    /// Private TlsConnection function that flushes TLS packets and handles any errors.
    fn do_tls_write_and_handle_error(&mut self) {
        let rc = self.tls_session.write_tls(&mut self.socket);
        if rc.is_err() {
            error!("write failed: {:?}", rc);
            self.closing = true;
            return;
        }
    }

    pub fn register(&mut self, registry: &mio::Registry) {
        let event_set = self.event_set();
        registry.register(&mut self.socket,
                          self.token,
                          event_set).unwrap();
    }

    fn reregister(&mut self, registry: &mio::Registry) {
        let event_set = self.event_set();
        registry.reregister(&mut self.socket,
                            self.token,
                            event_set).unwrap();
    }

    fn deregister(&mut self, registry: &mio::Registry) {
        registry.deregister(&mut self.socket).unwrap();
    }

    pub fn event_set(&self) -> mio::Interest {
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
