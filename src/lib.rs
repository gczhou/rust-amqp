#![desc = "Rust AMQP library"]
#![comment = "AMQP client"]
#![license = "MIT/BSD"]

#![feature(phase)]
extern crate collections;
extern crate url;
#[phase(plugin, link)] extern crate log;

pub mod connection;
pub mod channel;
pub mod framing;
pub mod table;
pub mod protocol;
pub mod session;
pub mod basic;
pub mod amqp_error;
