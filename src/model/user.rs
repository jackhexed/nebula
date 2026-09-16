use crate::{Snowflake, model::ModelCtx, snowflake::UserMarker};

/// Represents a Discord user.
#[derive(Clone)]
pub struct User {
    model_ctx: ModelCtx,
    user_id: Snowflake<UserMarker>,
}

impl From<User> for Snowflake<UserMarker> {
    fn from(value: User) -> Self {
        value.user_id
    }
}

impl From<&User> for Snowflake<UserMarker> {
    fn from(value: &User) -> Self {
        value.user_id
    }
}

impl User {
    /// Creates a new user model type.
    pub(crate) fn new(model_ctx: ModelCtx, user_id: Snowflake<UserMarker>) -> Self {
        Self { model_ctx, user_id }
    }

    /// Returns the user's snowflake.
    pub fn id(&self) -> Snowflake<UserMarker> {
        self.user_id
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}
