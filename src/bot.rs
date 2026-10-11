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
pub struct Bot<S = ()>
where
    S: Send + Sync + 'static,
{
    rest: Arc<Rest>,
    state: Arc<S>,
}

impl Bot<()> {
    /// Creates a new bot with the given token and application ID, both from
    /// the Discord developer portal.
    pub fn new(token: impl Into<String>, app_id: impl Into<Snowflake<AppMarker>>) -> Self {
        let state = Arc::new(());
        let rest = Rest::new(token.into(), app_id.into());

        Self { rest, state }
    }
}

impl<S> Bot<S>
where
    S: Send + Sync + 'static,
{
    /// Adds state to this bot, returning a new bot with the given state.
    pub fn with_state<Sp>(self, state: Sp) -> Bot<Sp>
    where
        Sp: Send + Sync + 'static,
    {
        Bot {
            rest: self.rest,
            state: Arc::new(state),
        }
    }

    /// Returns the REST client for this bot.
    pub(crate) fn rest(&self) -> Arc<Rest> {
        Arc::clone(&self.rest)
    }

    /// Returns the state for this bot.
    pub fn state(&self) -> Arc<S> {
        Arc::clone(&self.state)
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
    pub fn webhook<T>(&self, key: &str, callback: fn(Action<S>) -> T) -> Webhook<T, S>
    where
        T: Future<Output = ()> + Send + 'static,
    {
        Webhook::new(self, key, callback)
    }

    /// Creates a new gateway backend for this bot, with the given action
    /// callback.
    pub fn gateway<T>(&self, callback: fn(Action<S>) -> T) -> Gateway<T, S>
    where
        T: Future<Output = ()> + Send + 'static,
        S: Clone + Send + 'static,
    {
        Gateway::new(self, callback)
    }
}
