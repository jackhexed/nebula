use std::sync::Arc;

use crate::{Snowflake, snowflake::AppMarker};

mod tw {
    pub use twilight_http::client::{Client, InteractionClient};
}

/// The state shared by every action: the HTTP client and application ID.
///
/// This is the only route to Discord's REST API.
pub(crate) struct Shared {
    client: tw::Client,
    app_id: Snowflake<AppMarker>,
}

impl Shared {
    /// Creates shared state with the given token and application ID.
    pub(crate) fn new(token: String, app_id: Snowflake<AppMarker>) -> Arc<Self> {
        let client = tw::Client::new(token);

        Arc::new(Self { client, app_id })
    }

    /// Returns the Twilight HTTP client.
    pub(crate) fn client(&self) -> &tw::Client {
        &self.client
    }

    /// Returns an interaction client for the application.
    pub(crate) fn interaction(&self) -> tw::InteractionClient<'_> {
        self.client.interaction(self.app_id.into())
    }

    /// Returns the application ID.
    pub(crate) fn app_id(&self) -> Snowflake<AppMarker> {
        self.app_id
    }
}
