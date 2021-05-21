extern crate log;
extern crate simplelog;

#[cfg(feature = "client")]
use libtrader::client::initializer::libtrader_init_client;
#[cfg(feature = "server")]
use libtrader::server::initializer::libtrader_init_server;

fn main() {
    #[cfg(feature = "server")]
    libtrader_init_server().unwrap();

    /* this is a sandbox, we should try to atleast
     * implement a testing method */
    #[cfg(feature = "client")]
    libtrader_init_client().unwrap();
}
