use std::io;
use std::net::ToSocketAddrs;
use std::path::PathBuf;
use std::sync::Arc;

use argh::FromArgs;
use log::warn;

use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

use crate::server::network::gen_tls_server_config::gen_tls_server_config;

use crate::server::db::initializer::db_connect;
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

tokio::task_local! {
    pub static IP: std::net::SocketAddr;
}

/// Initializes global and local logger.
///
/// Private function used by libtrader_init() to initialize the logger. Log destinations are
/// platfrom dependent.
/// On unix systems: /var/log/papertrader/
/// On windows/unknown systems: $(pwd)/log/
/// Should be used in contexts that return ```io::Result```.
///
/// Returns: ```io::Result```.
///
/// Example:
/// ```rust
///     libtrader_init_log()?;
/// ```
///
fn libtrader_init_log() -> std::io::Result<()> {
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
            "{color_line}{date}[{addr}][{level}{color_line}] {message}\x1B[0m",
            color_line = format_args!(
                "\x1B[{}m",
                colors_level.get_color(&record.level()).to_fg_str()
            ),
            date = chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            addr = IP.get(),
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

/// Server Initialization of the library.
///
/// Public function that initializes the library, and starts the libtrader server.
///
/// Example:
/// ```rust
///     libtrader_init_server()?;
///     // Create tokio runtime
///     let rt = tokio::runtime::Builder::new_multi_thread()
///         .worker_threads(8)
///         .thread_name("libtrader_server_thread")
///         .enable_all()
///         .build()
///         .expect("failed creating server runtime");
///
///     // Spawn server
///     rt.block_on(async move {
///         IP.scope("0.0.0.0:0000".parse().unwrap(), async move {
///             // for main task logging
///             libtrader_init_server()
///                 .await
///                 .expect("failed running server");
///         })
///         .await;
///     });
/// ```
pub async fn libtrader_init_server() -> std::io::Result<()> {
    // Initialize log.
    libtrader_init_log()?;

    // Initialize SQL connection
    let sql_shared_conn = Arc::new(
        db_connect(
            std::env::var("DB_ACC_USER").unwrap(),
            std::env::var("DB_ACC_PASS").unwrap(),
        )
        .await
        .map_err(|err| {
            io::Error::new(
                io::ErrorKind::ConnectionAborted,
                format!("SQL_CONNECTION_FAILED: {}", err),
            )
        })?,
    );

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
        let (socket, peer_addr) = listener.accept().await?; // socket, peer_addr
        let acceptor = acceptor.clone();
        let sql_conn = sql_shared_conn.clone();

        // function to run in the thread
        let fut = async move {
            let mut socket = acceptor.accept(socket).await?;
            loop {
                let mut buf = Vec::with_capacity(4096);
                socket.read_buf(&mut buf).await?;
                match handle_data(&sql_conn, &mut socket, buf.as_slice()).await {
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
            IP.scope(peer_addr, async move {
                if let Err(err) = fut.await {
                    eprintln!("{:?}", err);
                }
            })
            .await;
        });
    }
}
