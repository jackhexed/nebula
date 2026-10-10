use crate::{Snowflake, ctx::Ctx, snowflake::UserMarker};

/// A Discord user.
///
/// See the [module docs](crate::model) for how models work.
///
/// [Discord docs](https://docs.discord.com/developers/resources/user#user-object-user-structure)
#[derive(Clone)]
pub struct User {
    ctx: Ctx,
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
    /// Creates a user model from its snowflake.
    pub(crate) fn new(ctx: impl Into<Ctx>, user_id: Snowflake<UserMarker>) -> Self {
        let ctx = ctx.into();

        Self { ctx, user_id }
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
