use crate::{Snowflake, model::ModelCtx, snowflake::GuildMarker};

/// Represents a Discord guild (server).
#[derive(Clone)]
pub struct Guild {
    model_ctx: ModelCtx,
    guild_id: Snowflake<GuildMarker>,
}

impl From<Guild> for Snowflake<GuildMarker> {
    fn from(value: Guild) -> Self {
        value.guild_id
    }
}

impl From<&Guild> for Snowflake<GuildMarker> {
    fn from(value: &Guild) -> Self {
        value.guild_id
    }
}

impl Guild {
    /// Creates a new guild model type.
    pub fn new(model_ctx: ModelCtx, guild_id: Snowflake<GuildMarker>) -> Self {
        Self {
            model_ctx,
            guild_id,
        }
    }

    /// Returns the guild's snowflake.
    pub fn id(&self) -> Snowflake<GuildMarker> {
        self.guild_id
    }
}

impl PartialEq for Guild {
    fn eq(&self, other: &Self) -> bool {
        self.guild_id == other.guild_id
    }
}
