#[cfg(any(feature="server", feature="client"))] #[macro_use] extern crate log;
#[cfg(any(feature="client", feature="client"))] #[macro_use] extern crate arrayref;

#[cfg(featue="server")] #[macro_use] extern crate lazy_static;
extern crate simplelog;
extern crate os_type;
extern crate bincode;
extern crate crypto;

pub mod common;

#[cfg(feature="server")]
pub mod server;
#[cfg(feature="client")] 
pub mod client;
