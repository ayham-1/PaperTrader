extern crate log;

#[cfg(feature = "client")]
use libtrader::client::initializer::libtrader_init_client;
#[cfg(feature = "server")]
use libtrader::server::initializer::{libtrader_init_server, IP};

fn main() {
    #[cfg(feature = "server")]
    {
        // Create tokio runtime
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(8)
            .thread_name("libtrader_server_thread")
            .enable_all()
            .build()
            .expect("failed creating server runtime");

        // Spawn server
        rt.block_on(async move {
            IP.scope("0.0.0.0:0000".parse().unwrap(), async move {
                // for main task logging
                libtrader_init_server()
                    .await
                    .expect("failed running server");
            })
            .await;
        });
    }

    /* this is a sandbox, we should try to atleast
     * implement a testing method */
    #[cfg(feature = "client")]
    libtrader_init_client().expect("failed running client");
}
