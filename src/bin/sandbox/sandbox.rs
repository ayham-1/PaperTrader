extern crate log;
extern crate simplelog;

#[cfg(feature="master_server")]
pub mod tls_server_sandbox;
#[cfg(feature="client")]
pub mod tls_client_sandbox;

use libtrader::account::sessions::jwt_wrapper::{create_jwt_token, verify_jwt_token};

fn main() {
    use std::time::{SystemTime, UNIX_EPOCH, Duration};
    let start = SystemTime::now() + Duration::from_secs(4*60*60);
    let token = 
        create_jwt_token(1usize, 
                         start.duration_since(UNIX_EPOCH)
                         .unwrap().as_secs() as usize).unwrap();
    println!("token: {}", token);

    println!("verified token: {:#?}", 
             verify_jwt_token(token).unwrap());

    #[cfg(feature="master_server")]
    tls_server_sandbox::tls_main();
    #[cfg(feature="client")]
    tls_client_sandbox::tls_main();
}

