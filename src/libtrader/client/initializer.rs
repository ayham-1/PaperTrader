use std::io;
use std::net::ToSocketAddrs;

use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use tokio_rustls::webpki::DNSNameRef;

use crate::common::misc::gen_tls_client_config::gen_tls_client_config;
use crate::common::misc::path_exists::path_exists;

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
fn libtrader_init_log() -> io::Result<()> {
    info!("Started Logger.");
    #[cfg(not(debug_assertions))]
    gen_log();

    #[cfg(debug_assertions)]
    {
        use simplelog::*;
        use std::fs::File;

        if !path_exists("log") {
            std::fs::create_dir("log")?;
        }
        CombinedLogger::init(vec![
            #[cfg(debug_assertions)]
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
            #[cfg(not(debug_assertions))]
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
            WriteLogger::new(
                LevelFilter::Info,
                Config::default(),
                File::create(format!("log/log-{}.txt", chrono::Utc::now().to_rfc2822())).unwrap(),
            ),
        ])
        .unwrap();
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
#[tokio::main]
pub async fn libtrader_init_client() -> std::io::Result<()> {
    match libtrader_init_log() {
        Ok(()) => {}
        Err(err) => return Err(err),
    };

    let addr = ("0.0.0.0", 4000).to_socket_addrs()?.next()
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;
    let domain = "localhost";
    let config = gen_tls_client_config()?;

    let connector = TlsConnector::from(config);
    let stream = TcpStream::connect(&addr).await?;

    let domain = DNSNameRef::try_from_ascii_str(&domain)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid dnsname"))?;

    let mut socket = connector.connect(domain, stream).await?;

    use crate::client::account::creation::acc_create;
    match acc_create(&mut socket, "test", "email", "password").await {
        Ok(_) => println!("we created it"),
        Err(err) => panic!("panik! {}", err),
    }

    use crate::client::account::authorization::acc_auth;
    let mut jwt: String = String::new();
    match acc_auth(&mut socket, "test", "email", "password").await {
        Ok(auth) => {
            jwt = auth;
            println!("we accessed it, the token: {}", jwt);
        },
        Err(err) => panic!("panik! {}", err),
    }

    use crate::client::account::retrieval_portfolio::acc_retrieve_portfolio;
    match acc_retrieve_portfolio(&mut socket, String::from(jwt.as_str())).await {
        Ok(portfolio) => println!("we got portfolio {:#?}", portfolio),
        Err(err) => panic!("panik! {}", err),
    }

    use crate::client::account::retrieval_transaction::acc_retrieve_transaction;
    match acc_retrieve_transaction(&mut socket, String::from(jwt.as_str())).await {
        Ok(transaction) => println!("we got the transactions {:#?}", transaction),
        Err(err) => panic!("panik! {}", err),
    }


    Ok(())
}
