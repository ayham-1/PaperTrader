use argh::FromArgs;
use std::path::PathBuf;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

use std::net::ToSocketAddrs;

use crate::common::misc::gen_tls_server_config::gen_tls_server_config;
use crate::common::misc::path_exists::path_exists;
use crate::common::misc::return_flags::ReturnFlags;
use crate::server::network::handle_data::handle_data;

/// Server Options
#[derive(FromArgs)]
struct Options {
    /// bind addr
    #[argh(positional)]
    addr: String,

    /// cert file
    #[argh(option, short = 'c')]
    cert: PathBuf,

    /// key file
    #[argh(option, short = 'k')]
    key: PathBuf,
}

#[cfg(not(debug_assertions))]
use crate::common::misc::gen_log::gen_log;

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
fn libtrader_init_log() -> Result<(), ReturnFlags> {
    info!("Started Logger.");
    #[cfg(not(debug_assertions))]
    gen_log();

    #[cfg(debug_assertions)]
    {
        use simplelog::*;
        use std::fs::File;

        if !path_exists("log") {
            match std::fs::create_dir("log") {
                Ok(()) => {}
                Err(_err) => return Err(ReturnFlags::CommonGenLogDirCreationFailed),
            };
        }
        CombinedLogger::init(vec![
            #[cfg(debug_assertions)]
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed),
            #[cfg(not(debug_assertions))]
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed),
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

/// Server Initialization of the library.
///
/// Public function that initializes the library, and starts the libtrader server.
/// This function should not return.
///
/// Example:
/// ```rust
///     libtrader_init_server()?;
/// ```
#[tokio::main]
pub async fn libtrader_init_server() -> std::io::Result<()> {
    // Initialize log.
    //#[cfg(not(test))] // wot dis
    match libtrader_init_log() {
        Ok(_) => {}
        Err(_) => {} // TODO: handle this case
    };

    // Initialize arguments
    let options: Options = argh::from_env();

    let addr = options
        .addr
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::AddrNotAvailable))?;

    let config = gen_tls_server_config(&options.cert, &options.key)?;
    let acceptor = TlsAcceptor::from(config);

    let listener = TcpListener::bind(&addr).await?;

    loop {
        let (socket, _) = listener.accept().await?; // socket, peer_addr
        let acceptor = acceptor.clone();

        // function to run in the thread
        let fut = async move {
            let mut socket = acceptor.accept(socket).await?;

            let mut buf = Vec::with_capacity(4096);
            loop {
                socket.read_buf(&mut buf).await?;
                match handle_data(&mut socket, buf.as_slice()).await {
                    Ok(()) => {}
                    Err(err) => {
                        warn!("{}", format!("Failed running handle_data: {:#?}", err));
                        break;
                    }
                };
            }

            Ok(()) as std::io::Result<()>
        };

        tokio::spawn(async move {
            if let Err(err) = fut.await {
                eprintln!("{:?}", err);
            }
        });
    }
}
