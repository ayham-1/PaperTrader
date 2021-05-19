#[cfg(any(feature = "server", feature = "client"))]
#[macro_use]
extern crate log;

#[cfg(all(feature = "server", not(feature = "client")))]
extern crate arrayref;
#[cfg(all(feature = "server", not(feature = "client")))]
extern crate json;

#[cfg(all(feature = "client", not(feature = "server")))]
#[macro_use]
extern crate arrayref;
#[cfg(all(feature = "client", not(feature = "server")))]
#[macro_use]
extern crate json;

#[cfg(all(feature = "server", feature = "client"))]
#[macro_use]
extern crate arrayref;
#[cfg(all(feature = "server", feature = "client"))]
#[macro_use]
extern crate json;

#[macro_use]
extern crate bitflags;

extern crate bincode;
extern crate crypto;
extern crate os_type;
extern crate simplelog;

pub mod common;

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "server")]
pub mod server;
