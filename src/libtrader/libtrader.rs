#[cfg(any(feature="server", feature="client"))] #[macro_use] extern crate log;
#[cfg(feature="server")] extern crate arrayref;
#[cfg(feature="server")] extern crate json;
#[cfg(feature="client")] #[macro_use] extern crate arrayref;
#[cfg(feature="client")] #[macro_use] extern crate json;

extern crate simplelog;
extern crate os_type;
extern crate bincode;
extern crate crypto;

pub mod common;

#[cfg(feature="server")]
pub mod server;
#[cfg(feature="client")] 
pub mod client;
