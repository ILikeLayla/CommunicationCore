// pub mod client;
// pub mod server;
mod service;
pub use service::start_service;

use super::net_protocol::{channel::Channel, replacer};
use super::data_class::FullMessage;