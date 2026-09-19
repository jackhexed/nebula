use std::sync::Arc;

use crate::{
    Snowflake,
    backend::ResponseSink,
    cache::Cache,
    model::{Guild, ModelCtx, User},
    shared::Shared,
    snowflake::ActionMarker,
};

/// The information every action shares.
pub struct Common {
    shared: Arc<Shared>,
    cache: Arc<Cache>,
    sink: ResponseSink,

    action_id: Snowflake<ActionMarker>,
    action_token: Box<str>,

    guild: Guild,
    user: User,
}

impl Common {
    /// Returns a new `Common` instance.
    pub(crate) fn new(
        shared: Arc<Shared>,
        cache: Arc<Cache>,
        sink: ResponseSink,
        action_id: Snowflake<ActionMarker>,
        action_token: Box<str>,
        guild: Guild,
        user: User,
    ) -> Self {
        Self {
            shared,
            cache,
            sink,
            action_id,
            action_token,
            guild,
            user,
        }
    }

    /// Returns the shared state.
    pub(crate) fn shared(&self) -> &Arc<Shared> {
        &self.shared
    }

    /// Returns the action's cache.
    pub(crate) fn cache(&self) -> &Arc<Cache> {
        &self.cache
    }

    /// Returns the action's response sink.
    pub(crate) fn sink(&self) -> &ResponseSink {
        &self.sink
    }

    /// Returns the action's snowflake.
    pub fn action_id(&self) -> Snowflake<ActionMarker> {
        self.action_id
    }

    /// Returns the action's token.
    ///
    /// The token is used to respond to the action, and is valid for 15
    /// minutes.
    pub fn action_token(&self) -> &str {
        &self.action_token
    }

    /// Returns the guild the action was performed in.
    pub fn guild(&self) -> &Guild {
        &self.guild
    }

    /// Returns the user who performed the action.
    pub fn user(&self) -> &User {
        &self.user
    }

    /// Returns a model context for the action.
    pub(crate) fn model_ctx(&self) -> ModelCtx {
        ModelCtx::new(Arc::clone(&self.shared), Arc::clone(&self.cache))
    }
}
