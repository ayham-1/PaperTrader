use std::sync::Arc;
use rustls;
use webpki_roots;
use ct_logs;

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
            Ok(rustls::ServerCertVerified::assertion())
        }
    }
}


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
