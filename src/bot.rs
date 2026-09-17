use crate::{Snowflake, action::Action, snowflake::AppMarker};

/// A bot's credentials and action handler.
///
/// A bot does nothing on its own. Pass it to a backend, such as
/// [`Gateway`](crate::backend::Gateway), to start receiving actions.
pub struct Bot<T>
where
    T: Future<Output = ()> + Send + 'static,
{
    /// The bot's token, from the Discord developer portal.
    pub token: String,

    /// The bot's application ID, from the Discord developer portal.
    pub app_id: Snowflake<AppMarker>,

    /// The function called with every action the bot receives.
    pub handler: fn(Action) -> T,
}
