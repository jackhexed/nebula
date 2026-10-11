use std::sync::Arc;

use crate::{
    Snowflake,
    cache::Cache,
    error::Error,
    model::{Guild, User},
    rest::Rest,
    slash::Slash,
    snowflake::{GuildMarker, UserMarker},
};

/// A context that holds the REST API client and cache.
#[derive(Clone)]
pub struct Ctx {
    rest: Arc<Rest>,
    cache: Arc<Cache>,
}

impl Ctx {
    /// Creates a new context with the given REST API client.
    ///
    /// This will create a new cache for the context.
    pub(crate) fn new(rest: Arc<Rest>) -> Self {
        let cache = Cache::new();

        Self { rest, cache }
    }

    /// Returns the REST API client.
    pub(crate) fn rest(&self) -> &Arc<Rest> {
        &self.rest
    }

    /// Returns the cache.
    pub(crate) fn cache(&self) -> &Arc<Cache> {
        &self.cache
    }

    /// Creates a guild with this context.
    pub fn guild(&self, guild_id: impl Into<Snowflake<GuildMarker>>) -> Guild {
        Guild::new(self, guild_id.into())
    }

    /// Creates a user with this context.
    pub fn user(&self, user_id: impl Into<Snowflake<UserMarker>>) -> User {
        User::new(self, user_id.into())
    }

    /// Set global commands.
    pub async fn set_global_commands(&self, commands: Vec<Slash>) -> Result<(), Error> {
        self.rest
            .interaction()
            .set_global_commands(&commands.into_iter().map(Into::into).collect::<Vec<_>>())
            .await?;

        Ok(())
    }
}

impl From<&Ctx> for Ctx {
    fn from(ctx: &Ctx) -> Self {
        ctx.clone()
    }
}
