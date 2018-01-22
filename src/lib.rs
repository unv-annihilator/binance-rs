#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![deny(missing_debug_implementations)]
// #![warn(missing_docs)]
#[macro_use]
extern crate error_chain;

extern crate hex;
extern crate log;
extern crate reqwest;
extern crate ring;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate tungstenite;
extern crate url;

pub mod errors;
pub mod model;
pub mod api;
