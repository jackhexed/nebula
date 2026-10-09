use std::sync::Arc;

use tokio_util::task::TaskTracker;

use crate::{
    Snowflake,
    action::Action,
    backend::{Gateway, ResponseSink, Webhook},
    rest::Rest,
    snowflake::AppMarker,
};

mod tw {
    pub use twilight_model::application::interaction::Interaction;
}

/// A bot's credentials and action handler.
pub struct Bot<T>
where
    T: Future<Output = ()> + Send + 'static,
{
    /// The bot's token, from the Discord developer portal.
    token: String,

    /// The bot's application ID, from the Discord developer portal.
    app_id: Snowflake<AppMarker>,

    /// The function called with every action the bot receives.
    handler: fn(Action) -> T,

    rest: Arc<Rest>,
    tracker: TaskTracker,
}

impl<T> Bot<T>
where
    T: Future<Output = ()> + Send + 'static,
{
    /// Creates a new bot with the given token, application ID, and action
    /// handler.
    pub fn new(token: String, app_id: Snowflake<AppMarker>, handler: fn(Action) -> T) -> Self {
        let rest = Rest::new(token.clone(), app_id);
        let tracker = TaskTracker::new();

        Self {
            token,
            app_id,
            handler,

            rest,
            tracker,
        }
    }

    /// Creates a new gateway backend for the bot.
    pub fn gateway(self) -> Gateway<T> {
        Gateway::new(self)
    }

    /// Creates a new webhook backend for the bot with the given public key.
    pub fn webhook(self, key: &str) -> Webhook<T> {
        Webhook::new(self, key)
    }

    /// Dispatches a new action to the bot's handler.
    pub(crate) fn dispatch(&self, sink: ResponseSink, interaction: tw::Interaction) {
        let rest = Arc::clone(&self.rest);
        let handler = self.handler;

        self.tracker.spawn(async move {
            handler(Action::parse(rest, sink, interaction)).await;
        });
    }

    /// Returns the bot's token.
    pub(crate) fn token(&self) -> &str {
        &self.token
    }

    /// Closes the bot's task tracker, waiting for all tasks to finish.
    pub(crate) async fn close(&self) {
        self.tracker.close();
        self.tracker.wait().await;
    }
}
