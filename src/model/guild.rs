use crate::{Snowflake, model::ModelCtx, snowflake::GuildMarker};

/// A Discord guild, also called a server.
///
/// See the [module docs](crate::model) for how models work.
///
/// [Discord docs](https://docs.discord.com/developers/resources/guild#guild-object-guild-structure)
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
    /// Creates a guild model from its snowflake.
    pub(crate) fn new(model_ctx: ModelCtx, guild_id: Snowflake<GuildMarker>) -> Self {
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
