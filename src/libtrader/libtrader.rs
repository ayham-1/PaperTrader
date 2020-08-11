#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate log;
#[macro_use] extern crate arrayref;
#[macro_use] extern crate lazy_static;
extern crate simplelog;
extern crate os_type;
extern crate bincode;
extern crate crypto;

pub mod common;

#[cfg(feature="server")]
pub mod server;
#[cfg(feature="client")]
pub mod client;
