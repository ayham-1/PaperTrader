use std::net;
use std::sync::Arc;
use mio;
use mio::net::TcpListener;

use std::fs;
use std::io::{Read, BufReader};

use rustls;

use rustls::{RootCertStore, NoClientAuth, AllowAnyAuthenticatedClient,
             AllowAnyAnonymousOrAuthenticatedClient};

use libtrader::network::tls_server::TlsServer;

#[derive(Debug, Default)]
struct Args {
    cmd_echo: bool,
    cmd_http: bool,
    cmd_forward: bool,
    flag_port: Option<u16>,
    flag_verbose: bool,
    flag_protover: Vec<String>,
    flag_suite: Vec<String>,
    flag_proto: Vec<String>,
    flag_certs: Option<String>,
    flag_key: Option<String>,
    flag_ocsp: Option<String>,
    flag_auth: Option<String>,
    flag_require_auth: bool,
    flag_resumption: bool,
    flag_tickets: bool,
    arg_fport: Option<u16>,
}

fn find_suite(name: &str) -> Option<&'static rustls::SupportedCipherSuite> {
    for suite in &rustls::ALL_CIPHERSUITES {
        let sname = format!("{:?}", suite.suite).to_lowercase();

        if sname == name.to_string().to_lowercase() {
            return Some(suite);
        }
    }

    None
}

fn lookup_suites(suites: &[String]) -> Vec<&'static rustls::SupportedCipherSuite> {
    let mut out = Vec::new();

    for csname in suites {
        let scs = find_suite(csname);
        match scs {
            Some(s) => out.push(s),
            None => panic!("cannot look up ciphersuite '{}'", csname),
        }
    }

    out
}

/// Make a vector of protocol versions named in `versions`
fn lookup_versions(versions: &[String]) -> Vec<rustls::ProtocolVersion> {
    let mut out = Vec::new();

    for vname in versions {
        let version = match vname.as_ref() {
            "1.2" => rustls::ProtocolVersion::TLSv1_2,
            "1.3" => rustls::ProtocolVersion::TLSv1_3,
            _ => panic!("cannot look up version '{}', valid are '1.2' and '1.3'", vname),
        };
        out.push(version);
    }

    out
}

fn load_certs(filename: &str) -> Vec<rustls::Certificate> {
    let certfile = fs::File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);
    rustls::internal::pemfile::certs(&mut reader).unwrap()
}

fn load_private_key(filename: &str) -> rustls::PrivateKey {
    let rsa_keys = {
        let keyfile = fs::File::open(filename)
            .expect("cannot open private key file");
        let mut reader = BufReader::new(keyfile);
        rustls::internal::pemfile::rsa_private_keys(&mut reader)
            .expect("file contains invalid rsa private key")
    };

    let pkcs8_keys = {
        let keyfile = fs::File::open(filename)
            .expect("cannot open private key file");
        let mut reader = BufReader::new(keyfile);
        rustls::internal::pemfile::pkcs8_private_keys(&mut reader)
            .expect("file contains invalid pkcs8 private key (encrypted keys not supported)")
    };

    // prefer to load pkcs8 keys
    if !pkcs8_keys.is_empty() {
        pkcs8_keys[0].clone()
    } else {
        assert!(!rsa_keys.is_empty());
        rsa_keys[0].clone()
    }
}

fn load_ocsp(filename: &Option<String>) -> Vec<u8> {
    let mut ret = Vec::new();

    if let &Some(ref name) = filename {
        fs::File::open(name)
            .expect("cannot open ocsp file")
            .read_to_end(&mut ret)
            .unwrap();
    }

    ret
}

fn make_config(args: &Args) -> Arc<rustls::ServerConfig> {
    let client_auth = if args.flag_auth.is_some() {
        let roots = load_certs(args.flag_auth.as_ref().unwrap());
        let mut client_auth_roots = RootCertStore::empty();
        for root in roots {
            client_auth_roots.add(&root).unwrap();
        }
        if args.flag_require_auth {
            AllowAnyAuthenticatedClient::new(client_auth_roots)
        } else {
            AllowAnyAnonymousOrAuthenticatedClient::new(client_auth_roots)
        }
    } else {
        NoClientAuth::new()
    };

    let mut config = rustls::ServerConfig::new(client_auth);
    config.key_log = Arc::new(rustls::KeyLogFile::new());

    let certs = load_certs(args.flag_certs.as_ref().expect("--certs option missing"));
    let privkey = load_private_key(args.flag_key.as_ref().expect("--key option missing"));
    let ocsp = load_ocsp(&args.flag_ocsp);
    config.set_single_cert_with_ocsp_and_sct(certs, privkey, ocsp, vec![])
        .expect("bad certificates/private key");

    if !args.flag_suite.is_empty() {
        config.ciphersuites = lookup_suites(&args.flag_suite);
    }

    if !args.flag_protover.is_empty() {
        config.versions = lookup_versions(&args.flag_protover);
    }

    if args.flag_resumption {
        config.set_persistence(rustls::ServerSessionMemoryCache::new(256));
    }

    if args.flag_tickets {
        config.ticketer = rustls::Ticketer::new();
    }

    config.set_protocols(&args.flag_proto
                         .iter()
                         .map(|proto| proto.as_bytes().to_vec())
                         .collect::<Vec<_>>()[..]);

    Arc::new(config)
}

pub fn tls_main() {
    let mut args = Args::default();
    args.flag_certs = Some("test_tls.crt".to_string());
    args.flag_key = Some("test_tls.key".to_string());
    args.flag_port = Some(4000);

    let mut addr: net::SocketAddr = "0.0.0.0:4000".parse().unwrap();
    addr.set_port(args.flag_port.unwrap_or(4000));

    let config = make_config(&args);

    let mut listener = TcpListener::bind(addr).expect("cannot listen on port");
    let mut poll = mio::Poll::new().unwrap();

    poll.registry().register(&mut listener, mio::Token(0), mio::Interest::READABLE).unwrap();

    let mut tlsserv = TlsServer::new(listener, config);

    let mut events = mio::Events::with_capacity(256);
    loop {
        poll.poll(&mut events, None).unwrap();

        for event in &events {
            match event.token() {
                mio::Token(0) => {
                    tlsserv.accept(poll.registry()).expect("error accepting socket");
                },
                _ => tlsserv.conn_event(poll.registry(), &event)
            }
        }
    }
}
