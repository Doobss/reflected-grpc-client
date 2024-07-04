extern crate protobuf;
extern crate thiserror;
extern crate tokio;
extern crate tokio_stream;
extern crate tonic;
extern crate tonic_reflection;
extern crate tracing;
extern crate tracing_subscriber;

#[cfg(feature = "cli")]
extern crate clap;

mod error;
pub mod logging;
pub mod reflection;

pub use error::{ReflectedClientError, ReflectedClientResult};
pub use reflection::{Client, ClientBuilder, Message, Method, Service};
