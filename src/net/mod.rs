pub mod client;
pub mod server;
mod service;

use super::net_protocol::{channel::Channel, replacer};
use super::data_class::FullMessage;