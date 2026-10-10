//! Contains backends, which receive actions from Discord and send back
//! responses.
mod dispatch;
mod gateway;
mod response;
mod webhook;

pub use gateway::Gateway;
pub use webhook::Webhook;

pub(crate) use response::{Response, ResponseKind, ResponseSink};
