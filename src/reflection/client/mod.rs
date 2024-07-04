#[allow(clippy::module_inception)]
mod client;
mod client_builder;
mod error;

pub use client::Client;
pub use client_builder::ClientBuilder;
pub use error::*;
