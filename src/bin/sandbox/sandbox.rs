extern crate log;
extern crate simplelog;

#[cfg(feature="master_server")]
pub mod tls_server_sandbox;
#[cfg(feature="client")]
pub mod tls_client_sandbox;

fn main() {
    #[cfg(feature="master_server")]
    tls_server_sandbox::tls_main();
    #[cfg(feature="client")]
    tls_client_sandbox::tls_main();
}

