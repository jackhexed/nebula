use std::sync::Arc;

use crate::{Snowflake, snowflake::AppMarker};

mod tw {
    pub use twilight_http::client::{Client, InteractionClient};
}

/// A shared state for the application.
pub(crate) struct Shared {
    client: tw::Client,
    app_id: Snowflake<AppMarker>,
}

impl Shared {
    /// Creates a new shared state with the given token and application ID.
    pub(crate) fn new(token: String, app_id: Snowflake<AppMarker>) -> Arc<Self> {
        let client = tw::Client::new(token);

        Arc::new(Self { client, app_id })
    }

    /// Returns a reference to the underlying Twilight HTTP client.
    pub(crate) fn client(&self) -> &tw::Client {
        &self.client
    }

    /// Returns a reference to the interaction client for the application.
    pub(crate) fn interaction(&self) -> tw::InteractionClient<'_> {
        self.client.interaction(self.app_id.into())
    }

    /// Returns the application ID associated with this shared state.
    pub(crate) fn app_id(&self) -> Snowflake<AppMarker> {
        self.app_id
    }
}
