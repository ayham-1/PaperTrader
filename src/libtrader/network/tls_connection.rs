use mio;
use mio::net::TcpStream;

struct TlsConnection {
    socket: TcpStream,
    token: mio::Token,
    closing: bool,
    closed: bool,
    tls_session: rustls::ServerSession,
}

impl TlsConnection {
    fn new(socket: TcpStream,
           token: mio::Token,
           tls_session: rustls::ServerSession)
        -> Connection {
            Connection {
                socket,
                token,
                closing: false,
                closed: false,
                tls_session,
            }
        }

    fn ready(&mut self, registry: &mio::Registry, ev: &mio::event::Event) {
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
            self.register(registry);
        }
    }

    fn do_tls_read(&mut self) {
        // read some tls data.
        let rc = self.tls_session.read_tls(&mut self.socket);
        if rc.is_err() {
            let err = rc.unwrap_err();

            if let io::ErrorKind::WouldBock = err.Kind() {
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

    fn incoming_plaintext(&mut self, buf: &[u8]) {
        /* TODO: handle the data. */
    }

    fn tls_write(&mut self) -> io::Result<usize> {
        self.tls_session.write_tls(&mut self.socket);
    }

    fn do_tls_write_and_handle_error(&mut self) {
        let rc = self.tls_write();
        if rc.is_err() {
            error!("write failed: {:?}", rc);
            self.closing = true;
            return;
        }
    }

    fn register(&mut self, registry: &mio::Registry) {
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

    fn event_set(&self) -> mio::Interest {
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
