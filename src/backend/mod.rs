//! Contains backends, which receive actions from Discord and send back
//! responses.
//!
//! Discord [delivers interactions](https://docs.discord.com/developers/interactions/overview)
//! either over the gateway or as HTTP requests to an interactions endpoint.
//! Nebula is built around the HTTP endpoint, served by a webhook backend that
//! does not exist yet. [`Gateway`] is for local testing only.

use std::cell::RefCell;

use tokio::sync::oneshot;

use crate::{Snowflake, error::Error, item::Msg, shared::Shared, snowflake::ActionMarker};

mod gateway;
mod webhook;

pub use gateway::Gateway;
pub use webhook::Webhook;

mod tw {
    pub use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
}

/// Delivers an action's response.
///
/// The webhook backend gives the sink a oneshot sender, and the response
/// becomes the reply to Discord's HTTP request. Without one, the response is
/// sent with a `create_response` REST call, which is what the gateway backend
/// uses.
pub(crate) struct ResponseSink {
    tx: RefCell<Option<oneshot::Sender<ResponseKind>>>,
}

impl ResponseSink {
    /// Creates a sink that sends through `tx`, or through the REST API if `tx`
    /// is `None`.
    pub(crate) fn new(tx: Option<oneshot::Sender<ResponseKind>>) -> Self {
        Self {
            tx: RefCell::new(tx),
        }
    }

    /// Sends a response.
    ///
    /// The first call consumes the oneshot sender, so later calls fall back to
    /// the REST API.
    ///
    /// # Errors
    ///
    /// This function will return an error if the oneshot receiver was dropped,
    /// or if the REST call fails.
    pub(crate) async fn send(&self, shared: &Shared, response: Response<'_>) -> Result<(), Error> {
        if let Some(tx) = self.tx.take() {
            tx.send(response.kind)
                .map_err(|_| Error::msg("oneshot closed unexpectedly"))
        } else {
            let id = response.action_id.into();
            let token = response.action_token;
            let response = response.kind.into();

            shared
                .interaction()
                .create_response(id, token, &response)
                .await?;

            Ok(())
        }
    }
}

/// A response, along with the action snowflake and token needed to send it
/// over REST.
pub(crate) struct Response<'action> {
    action_id: Snowflake<ActionMarker>,
    action_token: &'action str,

    kind: ResponseKind,
}

impl<'action> Response<'action> {
    /// Creates a response to the action with the given snowflake and token.
    pub(crate) fn new(
        action_id: Snowflake<ActionMarker>,
        action_token: &'action str,
        kind: ResponseKind,
    ) -> Self {
        Self {
            action_id,
            action_token,

            kind,
        }
    }
}

/// What an action is responded with.
pub(crate) enum ResponseKind {
    /// Reply with a message.
    Reply(Msg),

    /// Acknowledge the action now and reply later. The user sees a loading
    /// state until the reply is sent.
    DeferReply,
}

impl From<ResponseKind> for tw::InteractionResponse {
    fn from(value: ResponseKind) -> Self {
        match value {
            ResponseKind::Reply(msg) => Self {
                kind: tw::InteractionResponseType::ChannelMessageWithSource,
                data: Some(msg.into()),
            },

            ResponseKind::DeferReply => Self {
                kind: tw::InteractionResponseType::DeferredChannelMessageWithSource,
                data: None,
            },
        }
    }
}
