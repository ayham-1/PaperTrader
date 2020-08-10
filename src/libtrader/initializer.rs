#[allow(unused_imports)]
use crate::misc::gen_log::gen_log;
use crate::misc::path_exists::path_exists;
use crate::db::initializer::db_init;
use crate::ds::generic::global_state::GlobalState;

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

/// Generic Initialization of the library.
///
/// Public function that globaly initializes the library. Initializes log, and database.
///
/// Returns: ``GlobalState``` on success, and a string containing the reason
/// of failure.
///
/// Example:
/// ```rust
///     match libtrader_init() {
///         Ok(state) => println!("here is the initialized state: {}", state),
///         Err(err) => panic!("failed initializing libtrader, reason: {}", err)
///     };
/// ```
pub fn libtrader_init() -> Result<GlobalState, String> {
    let mut state: GlobalState = GlobalState::default();

    // Initialize log.
    #[cfg(not(test))]
    match libtrader_init_log() {
        Ok(()) => {},
        Err(err) => panic!("This should not happen!\n{}", err),
    };

    // Initialize database.
    match db_init(&mut state) {
        Ok(()) => info!("Initialized database."),
        Err(err) => return Err(format!("INIT_DB_FAILED: {}", err))
    }

    Ok(state)
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
#[cfg(feature="server")]
pub fn libtrader_init_server() -> Result<GlobalState, String> {
    use std::net;
    use mio::net::TcpListener;

    use crate::network::tls_server::TlsServer;
    use crate::misc::gen_tls_server_config::gen_tls_server_config;
    let _state: GlobalState = libtrader_init()?;
    
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

#[cfg(test)]
mod test {
    use crate::db::config::{DB_USER, DB_PASS};
    use crate::db::initializer::{db_connect};
    use crate::ds::generic::global_state::GlobalState;
    use crate::ds::generic::company::Company;

   use super::*;

    #[test]
    fn test_libtrader_init_log() {
        match libtrader_init_log() {
            Ok(()) => {},
            Err(err) => panic!("TEST_INIT_LOG_FAILED: {}", err)
        };
    }

    #[test]
    fn test_libtrader_init() {
        /* connect to db */
        let mut state: GlobalState = GlobalState::default();
        let mut client = db_connect(&mut state, DB_USER, DB_PASS).unwrap();

        /* add test compnay */
        let mut company = Company::default();
        company.id = 1234;
        company.symbol = "CPP".to_string();
        company.isin = "2".to_string();
        company.company_name = "CPP".to_string();
        company.primary_exchange = "NYSE".to_string();
        company.sector = "Tech".to_string();
        company.industry = "Tech".to_string();
        company.primary_sic_code = "2".to_string();
        company.employees = 1;
        client.execute(
            "INSERT INTO public.companies VALUES ($1,$2, $3, $4, $5, $6, $7, $8, $9)",
            &[&company.id, &company.symbol, &company.isin, &company.company_name, 
            &company.primary_exchange, &company.sector, &company.industry,
            &company.primary_sic_code, &company.employees]).unwrap();

        /* test libtrader_init */
        match libtrader_init() {
            Ok(state) => assert_eq!(state.companies.is_empty(), false),
            Err(err) => panic!("TEST_INIT_FAILED: {}", err)
        }
    }
}
