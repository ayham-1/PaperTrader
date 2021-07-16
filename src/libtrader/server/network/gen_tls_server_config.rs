use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

//use tokio_rustls::rustls::internal::pemfile::{certs, rsa_private_keys};
use tokio_rustls::rustls::internal::pemfile::{certs, pkcs8_private_keys, rsa_private_keys};
use tokio_rustls::rustls::{Certificate, NoClientAuth, PrivateKey, ServerConfig};

/// Loads a TLS public certificate
///
/// Arguments:
/// path - Path to .crt file.
///
/// Returns: ```io::Results``` wrapping vector of rustls' Certificate.
fn load_certs(path: &Path) -> std::io::Result<Vec<Certificate>> {
    certs(&mut BufReader::new(File::open(path)?))
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid cert"))
}

/// Load a TLS private key.
///
/// Arguments:
/// path - Path to .key file.
///
/// Returns: ```io::Results``` wrapping ```rustls::PrivateKey```.
fn load_private_keys(path: &Path) -> std::io::Result<Vec<PrivateKey>> {
    let rsa_keys = {
        let keyfile = File::open(path).expect("cannot open private key file");
        let mut reader = BufReader::new(keyfile);
        rsa_private_keys(&mut reader).expect("file contains invalid rsa private key")
    };

    let pkcs8_keys = {
        let keyfile = File::open(path).expect("cannot open private key file");
        let mut reader = BufReader::new(keyfile);
        pkcs8_private_keys(&mut reader)
            .expect("file contains invalid pkcs8 private key (encrypted keys not supported)")
    };

    // prefer to load pkcs8 keys
    if !pkcs8_keys.is_empty() {
        Ok(pkcs8_keys.clone())
    } else {
        assert!(!rsa_keys.is_empty());
        Ok(rsa_keys.clone())
    }
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
///
/// Returns: ```io::Result``` wrapping ```Arc<ServerConfig>```.
///
/// Example:
/// ```rust
///      let config = gen_tls_server_config("tests.crt", "priv.key")?;
/// ```
pub fn gen_tls_server_config(
    certs_file: &Path,
    priv_key_file: &Path,
) -> std::io::Result<Arc<ServerConfig>> {
    let mut config = ServerConfig::new(NoClientAuth::new());
    //config.key_log = Arc::new(rustls::KeyLogFile::new());

    /* load TLS certificate */
    let certs = load_certs(certs_file)?;
    let mut privkeys = load_private_keys(priv_key_file)?;
    config
        .set_single_cert(certs, privkeys.remove(0))
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))?;

    /* enable session resumption */
    //config.set_persistence(rustls::ServerSessionMemoryCache::new(512));
    //config.ticketer = rustls::Ticketer::new();

    Ok(Arc::new(config))
}
