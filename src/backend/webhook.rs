use bytes::Bytes;
use ed25519_dalek::{PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH, Signature, Verifier, VerifyingKey};
use hex::FromHex;
use tokio::sync::oneshot;

use crate::{
    Bot,
    action::Action,
    backend::{Dispatch, ResponseSink},
};

mod tw {
    pub use twilight_model::application::interaction::{Interaction, InteractionType};
    pub use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
}

/// A backend that handles Discord interactions through a webhook.
///
/// The webhook backend is designed to be used with a web server that receives
/// HTTP requests from Discord. It verifies the requests, parses the
/// interactions, and dispatches them to the bot for processing. The responses
/// are sent back to Discord through the HTTP response.
pub struct Webhook<T, S>
where
    T: Future<Output = ()> + Send + 'static,
    S: Send + Sync + 'static,
{
    dispatch: Dispatch<T, S>,
    key: VerifyingKey,
}

impl<T, S> Webhook<T, S>
where
    T: Future<Output = ()> + Send + 'static,
    S: Send + Sync + 'static,
{
    /// Creates a new webhook backend with the given bot, public key, action
    /// callback.
    ///
    /// State must be cloneable. Each action will receive a clone of the state.
    pub fn new(bot: &Bot<S>, key: &str, callback: fn(Action<S>) -> T) -> Self {
        let key = <[u8; PUBLIC_KEY_LENGTH]>::from_hex(key).expect("invalid public key");
        let key = VerifyingKey::from_bytes(&key).expect("invalid public key");

        let dispatch = Dispatch::new(bot.rest(), bot.state(), callback);

        Self { dispatch, key }
    }

    /// Stops the webhook backend.
    ///
    /// This method will wait until all in-flight actions are completed before
    /// returning. It is recommended to call this method when the server is
    /// shutting down.
    pub async fn stop(&self) {
        self.dispatch.stop().await;
    }

    /// Dispatches an HTTP request to the webhook backend.
    ///
    /// This method verifies the request, parses the interaction, and dispatches
    /// it to the bot for processing. The response is sent back to Discord
    /// through the HTTP response. If the request is invalid or the interaction
    /// cannot be processed, an appropriate HTTP status code is returned.
    pub async fn dispatch(&self, request: http::Request<Bytes>) -> http::Response<Bytes> {
        if !self.verify(&request) {
            return status(http::StatusCode::UNAUTHORIZED);
        }

        let Ok(interaction) = serde_json::from_slice::<tw::Interaction>(request.body()) else {
            return status(http::StatusCode::BAD_REQUEST);
        };

        if interaction.kind == tw::InteractionType::Ping {
            let response = tw::InteractionResponse {
                kind: tw::InteractionResponseType::Pong,
                data: None,
            };

            let Ok(json) = serde_json::to_vec(&response) else {
                return status(http::StatusCode::INTERNAL_SERVER_ERROR);
            };

            return http::Response::builder()
                .status(http::StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(json.into())
                .unwrap();
        }

        let (tx, rx) = oneshot::channel();
        let sink = ResponseSink::new(Some(tx));

        self.dispatch.send(sink, interaction);

        let Ok(response) = rx.await else {
            return status(http::StatusCode::INTERNAL_SERVER_ERROR);
        };

        let Ok(json) = serde_json::to_vec(&tw::InteractionResponse::from(response)) else {
            return status(http::StatusCode::INTERNAL_SERVER_ERROR);
        };

        http::Response::builder()
            .status(http::StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(json.into())
            .unwrap()
    }

    /// Verifies the signature of an HTTP request from Discord.
    fn verify(&self, request: &http::Request<Bytes>) -> bool {
        let body = request.body();

        let Some(signature) = request
            .headers()
            .get("X-Signature-Ed25519")
            .and_then(|s| <[u8; SIGNATURE_LENGTH]>::from_hex(s.as_bytes()).ok())
            .map(|s| Signature::from_bytes(&s))
        else {
            return false;
        };

        let Some(timestamp) = request
            .headers()
            .get("X-Signature-Timestamp")
            .map(|t| t.as_bytes())
        else {
            return false;
        };

        self.key
            .verify(&[timestamp, body].concat(), &signature)
            .is_ok()
    }
}

/// Creates an empty HTTP response with the given status code.
fn status(code: http::StatusCode) -> http::Response<Bytes> {
    http::Response::builder()
        .status(code)
        .body(Bytes::new())
        .unwrap()
}
