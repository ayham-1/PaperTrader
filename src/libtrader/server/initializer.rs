use std::net;
use mio::net::TcpListener;

use crate::common::misc::path_exists::path_exists;
use crate::common::misc::gen_tls_server_config::gen_tls_server_config;
use crate::common::misc::return_flags::ReturnFlags;

use crate::server::network::tls_server::TlsServer;

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

    #[cfg(debug_assertions)] {
        use simplelog::*;
        use std::fs::File;

        if !path_exists("log") {
            match std::fs::create_dir("log") {
                Ok(()) => {},
                Err(_err) => return Err(ReturnFlags::COMMON_GEN_LOG_DIR_CREATION_FAILED)
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

/// Server Initialization of the library.
///
/// Public function that initializes the library, and starts the libtrader server.
/// This function should not return.
///
/// Example:
/// ```rust
///     libtrader_init_server()?;
/// ```
pub fn libtrader_init_server() -> Result<(), ReturnFlags> {
    // Initialize log.
    #[cfg(not(test))]
    match libtrader_init_log() {
        Ok(()) => {},
        Err(err) => return Err(ReturnFlags::LIBTRADER_INIT_LOG_FAILED | err),
    };
    let addr: net::SocketAddr = "0.0.0.0:4000".parse().unwrap();
    let config = gen_tls_server_config("certs/test_tls.crt", "certs/test_tls.key", None);

    let mut listener = TcpListener::bind(addr).expect("LIBTRADER_INIT_SERVER_FAILED");
    let mut poll = mio::Poll::new().unwrap();

    poll.registry().register(&mut listener, mio::Token(0), mio::Interest::READABLE).unwrap();

    let mut tls_server = TlsServer::new(listener, config);
    let mut events = mio::Events::with_capacity(256);
    loop {
        poll.poll(&mut events, None).unwrap();

        for event in &events {
            match event.token() {
                mio::Token(0) => {
                    tls_server.accept(poll.registry()).expect("error accepting socket");
                },
                _ => tls_server.conn_event(poll.registry(), &event)
            }
        }
    }
}
