use std::cell::RefCell;

use tokio::sync::oneshot;

use crate::{Error, Snowflake, item::Msg, shared::Shared, snowflake::ActionMarker};

mod gateway;

pub use gateway::Gateway;

mod tw {
    pub use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
}

pub(crate) struct ResponseSink {
    tx: RefCell<Option<oneshot::Sender<ResponseKind>>>,
}

impl ResponseSink {
    pub(crate) fn new(tx: Option<oneshot::Sender<ResponseKind>>) -> Self {
        Self {
            tx: RefCell::new(tx),
        }
    }

    pub(crate) async fn send(&self, shared: &Shared, response: Response<'_>) -> Result<(), Error> {
        if let Some(tx) = self.tx.take() {
            tx.send(response.kind)
                .map_err(|_| Error::new("oneshot closed unexpectedly"))
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

pub(crate) struct Response<'action> {
    action_id: Snowflake<ActionMarker>,
    action_token: &'action str,

    kind: ResponseKind,
}

impl<'action> Response<'action> {
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

pub(crate) enum ResponseKind {
    Reply(Msg),
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
