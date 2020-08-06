use std::sync::Arc;
use rustls;
use webpki_roots;
use ct_logs;

/// A "always accept" Certficate verifier.
///
/// WARN: only to be used in development environments.
/// using this in production envs will be a HUGE security vuln.
#[cfg(feature = "tls_no_verify")]
mod danger {
    use super::rustls;
    use webpki;

    pub struct NoCertificateVerification {}

    impl rustls::ServerCertVerifier for NoCertificateVerification {
        fn verify_server_cert(&self,
                              _roots: &rustls::RootCertStore,
                              _presented_certs: &[rustls::Certificate],
                              _dns_name: webpki::DNSNameRef<'_>,
                              _ocsp: &[u8]) -> Result<rustls::ServerCertVerified, rustls::TLSError> {
            warn!("IF THIS IS NOT A DEV BUILD DO NOT PROCEED");
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
/// - Certificate Authentication.
/// - No persistent session storage.
///
/// Returns; the client configuration in an Arc.
pub fn gen_tls_client_config() -> Arc<rustls::ClientConfig> {
    let mut config = rustls::ClientConfig::new();
    config.key_log = Arc::new(rustls::KeyLogFile::new());

    config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
    config.ct_logs = Some(&ct_logs::LOGS);

    let persist = Arc::new(rustls::NoClientSessionStorage {});
    config.set_persistence(persist);

    #[cfg(feature = "tls_no_verify")]
    config.dangerous().set_certificate_verifier(Arc::new(danger::NoCertificateVerification {}));

    Arc::new(config)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gen_tls_cleint_config() {
        let _ = gen_tls_client_config();
    }
}
