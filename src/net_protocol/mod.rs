mod replacer;
mod manager;
mod mg_handler;
mod channel;

pub use channel::Channel;
pub use manager::MgManager;
pub use replacer::{Outputer, Replace};

use super::data_class;