use mio::net::TcpStream;

use crate::common::misc::path_exists::path_exists;
use crate::common::misc::gen_tls_client_config::gen_tls_client_config;
use crate::common::misc::lookup_ipv4::lookup_ipv4;

use crate::client::network::tls_client::TlsClient;

/// Initializes global logger.
///
/// Private function used by libtrader_init() to initialize the logger. Log destinations are
/// platfrom dependent.
/// On unix systems: /var/log/papertrader/
/// On windows/unkown systems: $(pwd)/log/
///
/// Returns: nothing on success, on error contains the reason of failure.
///
/// Example:
/// ```rust
///     match libtrader_init_log() {
///         Ok(()) => {},
///         Err(err) => panic!("failed initializing log, reason: {}", err)
///     };
/// ```
///
fn libtrader_init_log() -> Result<(), String> {
    info!("Started Logger.");
    #[cfg(not(debug_assertions))]
    gen_log();

    #[cfg(debug_assertions)] {
        use simplelog::*;
        use std::fs::File;

        if !path_exists("log") {
            match std::fs::create_dir("log") {
                Ok(()) => {},
                Err(err) => panic!("GEN_LOG_FAILED_DIR_CREATION: {}", err)
            };
        }
        CombinedLogger::init(vec![
                             #[cfg(debug_assertions)]
                             TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
                             #[cfg(not(debug_assertions))]
                             TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
                             WriteLogger::new(LevelFilter::Info, Config::default(), 
                                              File::create(format!("log/log-{}.txt", 
                                                                   chrono::Utc::now().to_rfc2822())).unwrap())
        ]).unwrap();
    };

    Ok(())
}

/// Client Initialization of the library.
///
/// Public function that initializes the library, and connects to a libtrader server
/// This funciton should not return.
///
/// Example:
/// ```rust
///     libtrader_init_client()?;
/// ```
pub fn libtrader_init_client() -> Result<(), String> {
    #[cfg(not(test))]
    match libtrader_init_log() {
        Ok(()) => {},
        Err(err) => panic!("This should not happen!\n{}", err),
    };

    let addr = lookup_ipv4("0.0.0.0", 4000);
    let config = gen_tls_client_config();
    
    let sock = match TcpStream::connect(addr) {
        Ok(socket) => socket,
        Err(err) => {
            error!("LIBTRADER_INIT_CLIENT_CONNECT_FAILED: {}", err);
            return Err("could not connect to server!".to_string());
        }
    };
    let dns_name = webpki::DNSNameRef::try_from_ascii_str("localhost").unwrap();
    let mut tls_client = TlsClient::new(sock, dns_name, config);

    let mut poll = mio::Poll::new().unwrap();
    let mut events = mio::Events::with_capacity(32);
    tls_client.register(poll.registry());

    loop {
        poll.poll(&mut events, None).unwrap();
        println!("KHELLO");

        for ev in &events {
            tls_client.ready(&ev);
            tls_client.reregister(poll.registry());

            //use crate::client::account::creation::acc_create;
            //match acc_create(&mut tls_client, &mut poll, "test", "email", "password") {
            //    Ok(_) => println!("we created it"),
            //    Err(err) => panic!("panik! {}", err),
            //}

            use crate::client::account::authorization::acc_auth;
            match acc_auth(&mut tls_client, &mut poll, "test", "email", "password") {
                Ok(_) => println!("we accessed it, the token: {}", tls_client.auth_jwt),
                Err(err) => panic!("panik! {}", err),
            }

            use crate::client::account::retrieval_portfolio::acc_retrieve_portfolio;
            match acc_retrieve_portfolio(&mut tls_client, &mut poll) {
                Ok(portfolio) => println!("we got portfolio {:#?}", portfolio),
                Err(err) => panic!("panik! {}", err),
            }
        }
    }
}

#[cfg(test)]
mod test {
   use super::*;

    #[test]
    fn test_libtrader_init_log() {
        match libtrader_init_log() {
            Ok(()) => {},
            Err(err) => panic!("TEST_INIT_LOG_FAILED: {}", err)
        };
    }
}
