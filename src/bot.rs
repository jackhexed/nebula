use crate::{Snowflake, action::Action, snowflake::AppMarker};

pub struct Bot<T>
where
    T: Future<Output = ()> + Send + 'static,
{
    pub token: String,
    pub app_id: Snowflake<AppMarker>,
    pub handler: fn(Action) -> T,
}
