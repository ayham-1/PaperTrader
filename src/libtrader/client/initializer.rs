use std::io;
use std::net::ToSocketAddrs;

use tokio::net::TcpStream;
use tokio_rustls::webpki::DNSNameRef;
use tokio_rustls::TlsConnector;

use crate::client::network::gen_tls_client_config::gen_tls_client_config;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

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
    use fern::colors::{Color, ColoredLevelConfig};

    let mut dispatch = fern::Dispatch::new().format(|out, message, record| {
        // configure colors for the whole line
        let colors_line = ColoredLevelConfig::new()
            .error(Color::Red)
            .warn(Color::White)
            // we actually don't need to specify the color for debug and info, they are white by default
            .info(Color::Green)
            .debug(Color::Yellow)
            // depending on the terminals color scheme, this is the same as the background color
            .trace(Color::BrightBlack);

        // configure colors for the name of the level.
        // since almost all of them are the same as the color for the whole line, we
        // just clone `colors_line` and overwrite our changes
        let colors_level = colors_line.clone().info(Color::Green);

        out.finish(format_args!(
            "{color_line}{date}[{target}][{level}{color_line}] {message}\x1B[0m",
            color_line = format_args!(
                "\x1B[{}m",
                colors_level.get_color(&record.level()).to_fg_str()
            ),
            date = chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            target = record.target(),
            level = record.level(),
            message = message
        ))
    });
    #[cfg(debug_assertions)]
    {
        dispatch = dispatch
            .level(log::LevelFilter::Debug)
            .chain(std::io::stdout());
    }
    #[cfg(not(debug_assertions))]
    {
        dispatch = dispatch
            .level(log::LevelFilter::Warn)
            .chain(std::io::stdout())
            .chain(fern::log_file(format!(
                "log/log-{}.log",
                chrono::Utc::now().to_rfc2822()
            ))?);
    }
    dispatch.apply().map_err(|err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("LIBTRADER_INIT_SERVER_LOG_FAILED: {}", err),
        )
    })
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
    // Initialize log.
    libtrader_init_log()?;

    let addr = ("0.0.0.0", 4000)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;
    let domain = "localhost";
    let config = gen_tls_client_config()?;

    let connector = TlsConnector::from(config);
    let stream = TcpStream::connect(&addr).await?;

    let domain = DNSNameRef::try_from_ascii_str(&domain)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid dnsname"))?;

    let mut socket = connector.connect(domain, stream).await?;

    let username: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    let email: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    use crate::client::account::creation::acc_create;
    match acc_create(&mut socket, &username, &email, &password).await {
        Ok(_) => println!("we created it"),
        Err(err) => panic!("panik! {}", err),
    }

    use crate::client::account::authorization::acc_auth;
    let mut jwt: String = String::new();
    println!("{}", jwt); // this is for removing pisky warnings,
                         // this is fine as long as this code is sandbox
    match acc_auth(&mut socket, &username, &email, &password).await {
        Ok(auth) => {
            jwt = auth;
            println!("we accessed it, the token: {}", jwt);
        }
        Err(err) => panic!("panik! {}", err),
    }

    use crate::client::account::retrieval_portfolio::acc_retrieve_portfolio;
    match acc_retrieve_portfolio(&mut socket, String::from(jwt.as_str())).await {
        Ok(portfolio) => println!("we got portfolio {:#?}", portfolio),
        Err(err) => panic!("panik! {}", err),
    }

    use crate::client::account::retrieval_transaction::acc_retrieve_transaction;
    match acc_retrieve_transaction(&mut socket, jwt).await {
        Ok(transaction) => println!("we got the transactions {:#?}", transaction),
        Err(err) => panic!("panik! {}", err),
    }

    Ok(())
}
