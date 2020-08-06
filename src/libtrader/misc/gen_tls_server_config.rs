use std::fs;
use std::io::{Read, BufReader};
use std::sync::Arc;
use rustls;
use rustls::NoClientAuth;

/// Loads a TLS public certificate
///
/// Arguments:
/// filename - Path to .crt file.
///
/// Returns: vector of rustls' Certificate
fn load_certs(filename: &str) -> Vec<rustls::Certificate> {
    let certfile = fs::File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);
    rustls::internal::pemfile::certs(&mut reader).unwrap()
}

/// Load a TLS private key.
///
/// Arguments:
/// filename - Path to .key file.
///
/// Returns: rustls::PrivateKey
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

/// Loads OCSP stapling key.
///
/// Argument:
/// filename - path to OCSP stapling key.
///
/// Returns: u8 vec
fn load_ocsp(filename: &Option<&str>) -> Vec<u8> {
    let mut ret = Vec::new();

    if let &Some(ref name) = filename {
        fs::File::open(name)
            .expect("cannot open ocsp file")
            .read_to_end(&mut ret)
            .unwrap();
    }
    ret
}

/// Generates a TlsServer Config.
///
/// Uses defualt settings for:
/// - TLS Protocol Version.
/// - TLS Protocol CypherSuite
/// - Whether to use OCSP or not.
///
/// Assumed Settings:
/// - TLS session resumption.
/// - TLS session ticketing.
///
/// Arguments:
/// certs_file - public certificate path.
/// priv_key_file - private key for the public certificiate path.
/// ocsp_key_file - ocsp certificate for public certificate path.
///
/// Returns: the server configuration in an Arc
///
/// Example:
/// ```rust
///      let config = gen_tls_server_config("tests.crt", "priv.key", None);
/// ```
pub fn gen_tls_server_config(certs_file: &str, priv_key_file: &str, ocsp_key_file: Option<&str>) -> Arc<rustls::ServerConfig> {
    let mut config = rustls::ServerConfig::new(NoClientAuth::new());
    config.key_log = Arc::new(rustls::KeyLogFile::new());

    /* load TLS certificate */
    let certs = load_certs(certs_file);
    let privkey = load_private_key(priv_key_file);
    let ocsp = load_ocsp(&ocsp_key_file); 
    config.set_single_cert_with_ocsp_and_sct(certs, privkey, ocsp, vec![]).expect("bad certs/priv key");

    /* enable session resumption */
    config.set_persistence(rustls::ServerSessionMemoryCache::new(512));
    config.ticketer = rustls::Ticketer::new();

    Arc::new(config)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gen_tls_server_config() {
        let _ = gen_tls_server_config("test_tls.crt", "test_tls.key", None);
    }
}
