mod service;

pub use service::start_service;

use super::net_protocol::{Channel, Replace};
use super::data_class::FullMessage;