use crate::{
    Snowflake,
    model::ModelCtx,
    snowflake::{GuildMarker, RoleMarker},
};

/// A role in a Discord guild.
///
/// See the [module docs](crate::model) for how models work.
///
/// [Discord docs](https://docs.discord.com/developers/topics/permissions#role-object-role-structure)
#[derive(Clone)]
pub struct Role {
    model_ctx: ModelCtx,
    guild_id: Snowflake<GuildMarker>,
    role_id: Snowflake<RoleMarker>,
}

impl From<Role> for Snowflake<RoleMarker> {
    fn from(value: Role) -> Self {
        value.role_id
    }
}

impl From<&Role> for Snowflake<RoleMarker> {
    fn from(value: &Role) -> Self {
        value.role_id
    }
}

impl Role {
    /// Creates a role model from its guild's snowflake and its own.
    pub(crate) fn new(
        model_ctx: ModelCtx,
        guild_id: Snowflake<GuildMarker>,
        role_id: Snowflake<RoleMarker>,
    ) -> Self {
        Self {
            model_ctx,
            guild_id,
            role_id,
        }
    }

    /// Returns the role's snowflake.
    pub fn id(&self) -> Snowflake<RoleMarker> {
        self.role_id
    }
}

impl PartialEq for Role {
    fn eq(&self, other: &Self) -> bool {
        self.role_id == other.role_id
    }
}
