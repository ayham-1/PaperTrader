use std::sync::Arc;

use tokio_rustls::rustls::{ClientConfig, KeyLogFile, NoClientSessionStorage};

/// A "always accept" certficate verifier.
///
/// WARN: only to be used in development environments.
/// using this in production envs WILL BE A HUGE security vuln.
#[cfg(feature = "tls_no_verify")]
mod danger {
    use log::warn;
    use tokio_rustls::rustls;
    use tokio_rustls::webpki;
    pub struct NoCertificateVerification {}

    impl rustls::ServerCertVerifier for NoCertificateVerification {
        fn verify_server_cert(
            &self,
            _roots: &rustls::RootCertStore,
            _presented_certs: &[rustls::Certificate],
            _dns_name: webpki::DNSNameRef<'_>,
            _ocsp: &[u8],
        ) -> Result<rustls::ServerCertVerified, rustls::TLSError> {
            warn!("IF THIS IS NOT A DEVELOPER BUILD DO NOT PROCEED");
            Ok(rustls::ServerCertVerified::assertion())
        }
    }
}

/// Generates a TlsClient Config.
///
/// Uses defualt settings for:
/// - TLS Protocol Version.
/// - TLs Protocol CypherSuite.
///
/// Assumed Settings:
/// - if cargo feature "tls_no_verify", then certificates are not
/// checked, else they are.
/// - No persistent session storage.
///
/// Returns; the client configuration in an ```std::io::Result<Arc<ClientConfig>>```.
/// Example already present in rustls' examples on github.
pub fn gen_tls_client_config() -> std::io::Result<Arc<ClientConfig>> {
    let mut config = ClientConfig::new();
    config.key_log = Arc::new(KeyLogFile::new());

    config
        .root_store
        .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
    config.ct_logs = Some(&ct_logs::LOGS);

    let persist = Arc::new(NoClientSessionStorage {});
    config.set_persistence(persist);

    #[cfg(feature = "tls_no_verify")]
    config
        .dangerous()
        .set_certificate_verifier(Arc::new(danger::NoCertificateVerification {}));

    Ok(Arc::new(config))
}
