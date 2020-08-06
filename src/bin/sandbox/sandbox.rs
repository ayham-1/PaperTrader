extern crate log;
extern crate simplelog;

pub mod tls_sandbox;
use crate::tls_sandbox::tls_main;

fn main() {
        tls_main();
}

