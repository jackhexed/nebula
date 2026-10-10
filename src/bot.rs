use std::sync::Arc;

use crate::{
    Snowflake,
    action::Action,
    backend::{Gateway, Webhook},
    rest::Rest,
    snowflake::AppMarker,
};

/// A bot, identified by its token and application ID.
///
/// Backends created from the same bot share its REST client.
pub struct Bot {
    rest: Arc<Rest>,
}

impl Bot {
    /// Creates a new bot with the given token and application ID, both from
    /// the Discord developer portal.
    pub fn new(token: String, app_id: Snowflake<AppMarker>) -> Self {
        let rest = Rest::new(token, app_id);

        Self { rest }
    }

    /// Returns the REST client for this bot.
    pub(crate) fn rest(&self) -> Arc<Rest> {
        self.rest.clone()
    }

    /// Creates a new webhook backend for this bot, with the given public key
    /// and action callback.
    pub fn webhook<T>(&self, key: &str, callback: fn(Action) -> T) -> Webhook<T>
    where
        T: Future<Output = ()> + Send + 'static,
    {
        Webhook::new(self, key, callback)
    }

    /// Creates a new gateway backend for this bot, with the given action
    /// callback.
    pub fn gateway<T>(&self, callback: fn(Action) -> T) -> Gateway<T>
    where
        T: Future<Output = ()> + Send + 'static,
    {
        Gateway::new(self, callback)
    }
}
