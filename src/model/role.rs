use crate::{
    Snowflake,
    model::ModelCtx,
    snowflake::{GuildMarker, RoleMarker},
};

/// Represents a Discord role.
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
    /// Creates a new role model type.
    pub fn new(
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
