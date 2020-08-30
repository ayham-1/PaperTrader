#[cfg(any(feature="server", feature="client"))] #[macro_use] extern crate log;
#[cfg(any(feature="server", feature="client"))] #[macro_use] extern crate bitflags;

#[cfg(all(feature="server", not(feature="client")))] extern crate arrayref;
#[cfg(all(feature="server", not(feature="client")))] extern crate json;

#[cfg(all(feature="client", not(feature="server")))] #[macro_use] extern crate arrayref;
#[cfg(all(feature="client", not(feature="server")))] #[macro_use] extern crate json;


#[cfg(all(feature="server", feature="client"))] #[macro_use] extern crate arrayref;
#[cfg(all(feature="server", feature="client"))] #[macro_use] extern crate json;

extern crate simplelog;
extern crate os_type;
extern crate bincode;
extern crate crypto;

pub mod common;

#[cfg(feature="server")]
pub mod server;
#[cfg(feature="client")] 
pub mod client;
