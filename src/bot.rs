use std::sync::Arc;

use crate::{
    Ctx, Snowflake,
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

    /// Creates a one-off context for this bot.
    ///
    /// This context is meant to be used for processes that do not have an
    /// associated action, such as sending a message to a channel on a schedule.
    pub fn ctx(&self) -> Ctx {
        Ctx::new(Arc::clone(&self.rest))
    }

    /// Creates a new webhook backend for this bot, with the given public key
    /// and action callback.
    pub fn webhook<T, S>(&self, key: &str, state: S, callback: fn(Action, S) -> T) -> Webhook<T, S>
    where
        T: Future<Output = ()> + Send + 'static,
        S: Clone + Send + 'static,
    {
        Webhook::new(self, key, state, callback)
    }

    /// Creates a new gateway backend for this bot, with the given action
    /// callback.
    pub fn gateway<T, S>(&self, state: S, callback: fn(Action, S) -> T) -> Gateway<T, S>
    where
        T: Future<Output = ()> + Send + 'static,
        S: Clone + Send + 'static,
    {
        Gateway::new(self, state, callback)
    }
}
