use std::sync::Arc;

use crate::{Snowflake, snowflake::AppMarker};

mod tw {
    pub use twilight_http::client::{Client, InteractionClient};
}

/// The REST API client for the bot, along with the bot's credentials.
///
/// This is the only route to Discord's REST API.
pub(crate) struct Rest {
    token: String,
    app_id: Snowflake<AppMarker>,

    client: tw::Client,
}

impl Rest {
    /// Creates a new REST API client with the given token and application ID.
    pub(crate) fn new(token: String, app_id: Snowflake<AppMarker>) -> Arc<Self> {
        let client = tw::Client::new(token.clone());

        Arc::new(Self {
            token,
            app_id,
            client,
        })
    }

    /// Returns the bot's token.
    pub(crate) fn token(&self) -> &str {
        &self.token
    }

    /// Returns the application ID.
    pub(crate) fn app_id(&self) -> Snowflake<AppMarker> {
        self.app_id
    }

    /// Returns the Twilight HTTP client.
    pub(crate) fn client(&self) -> &tw::Client {
        &self.client
    }

    /// Returns an interaction client for the application.
    pub(crate) fn interaction(&self) -> tw::InteractionClient<'_> {
        self.client.interaction(self.app_id.into())
    }
}
