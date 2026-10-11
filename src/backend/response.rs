use std::{mem, sync::Mutex};

use tokio::sync::oneshot;

use crate::{Snowflake, error::Error, item::Msg, rest::Rest, snowflake::ActionMarker};

mod tw {
    pub use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
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

    /// Creates the first response over REST.
    async fn first(self, rest: &Rest) -> Result<(), Error> {
        rest.interaction()
            .create_response(self.action_id.into(), self.action_token, &self.kind.into())
            .await?;

        Ok(())
    }

    /// All responses after the initial one are sent over REST.
    async fn after(self, rest: &Rest) -> Result<(), Error> {
        match self.kind {
            ResponseKind::Reply(msg) => {
                rest.interaction()
                    .update_response(self.action_token)
                    .flags((&msg).into())
                    .components(Some(&Box::from(msg)))
                    .await?;
            }

            ResponseKind::DeferReply => {
                panic!("cannot send a DeferReply after the initial response");
            }

            ResponseKind::Edit(msg) => {
                rest.interaction()
                    .update_response(self.action_token)
                    .flags((&msg).into())
                    .components(Some(&Box::from(msg)))
                    .await?;
            }

            ResponseKind::DeferEdit => {
                panic!("cannot send a DeferEdit after the initial response");
            }
        }

        Ok(())
    }
}

/// What an action is responded with.
pub(crate) enum ResponseKind {
    /// Reply with a message.
    Reply(Msg),

    /// Acknowledge the action now and reply later. The user sees a loading
    /// state until the reply is sent.
    DeferReply,

    /// Edit the original reply with a new message.
    Edit(Msg),

    /// Defer the edit of the original reply.
    DeferEdit,
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

            ResponseKind::Edit(msg) => Self {
                kind: tw::InteractionResponseType::UpdateMessage,
                data: Some(msg.into()),
            },

            ResponseKind::DeferEdit => Self {
                kind: tw::InteractionResponseType::DeferredUpdateMessage,
                data: None,
            },
        }
    }
}

/// Delivers an action's response.
///
/// The webhook backend gives the sink a oneshot sender, and the response
/// becomes the reply to Discord's HTTP request. Without one, the response is
/// sent with a `create_response` REST call, which is what the gateway backend
/// uses.
pub(crate) struct ResponseSink {
    state: Mutex<ResponseSinkState>,
}

impl ResponseSink {
    /// Creates a sink that sends through `tx`, or through the REST API if `tx`
    /// is `None`.
    pub(crate) fn new(tx: Option<oneshot::Sender<ResponseKind>>) -> Self {
        let state = Mutex::new(ResponseSinkState::Pending(tx));

        Self { state }
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
    pub(crate) async fn send(&self, rest: &Rest, response: Response<'_>) -> Result<(), Error> {
        let state = mem::replace(
            &mut *self.state.lock().unwrap(),
            ResponseSinkState::Responded,
        );

        match state {
            ResponseSinkState::Pending(Some(tx)) => tx
                .send(response.kind)
                .map_err(|_| Error::msg("oneshot closed unexpectedly")),

            ResponseSinkState::Pending(None) => response.first(rest).await,

            ResponseSinkState::Responded => response.after(rest).await,
        }
    }
}

enum ResponseSinkState {
    Pending(Option<oneshot::Sender<ResponseKind>>),
    Responded,
}
