use std::{future::Pending, time::Duration};

use tokio::time::Instant;
use twilight_gateway::StreamExt;

use crate::{
    Bot,
    action::Action,
    backend::{Dispatch, ResponseSink},
    error::Error,
};

mod tw {
    pub use twilight_gateway::{CloseFrame, Event, EventTypeFlags, Intents, Shard, ShardId};
}

/// A backend that receives actions over the Discord gateway.
///
/// This backend is intended for local testing, not for production use. It does
/// not support sharding and only listens for interaction events.
pub struct Gateway<T, S = Pending<()>>
where
    T: Future<Output = ()> + Send + 'static,
    S: Future<Output = ()> + Send + 'static,
{
    token: String,
    dispatch: Dispatch<T>,
    shutdown: S,
}

impl<T> Gateway<T, Pending<()>>
where
    T: Future<Output = ()> + Send + 'static,
{
    /// Creates a new gateway backend with the given bot and action callback.
    pub fn new(bot: &Bot, callback: fn(Action) -> T) -> Self {
        let token = bot.rest().token().to_owned();
        let dispatch = Dispatch::new(bot.rest(), callback);
        let shutdown = std::future::pending();

        Self {
            token,
            dispatch,
            shutdown,
        }
    }
}

impl<T, S> Gateway<T, S>
where
    T: Future<Output = ()> + Send + 'static,
    S: Future<Output = ()> + Send + 'static,
{
    /// Attaches a shutdown future to the gateway backend.
    ///
    /// When the future completes, the gateway will gracefully close its
    /// connection to Discord and stop receiving events.
    pub fn with_shutdown<Sp>(self, shutdown: Sp) -> Gateway<T, Sp>
    where
        Sp: Future<Output = ()> + Send + 'static,
    {
        let token = self.token;
        let dispatch = self.dispatch;

        Gateway {
            token,
            dispatch,
            shutdown,
        }
    }

    /// Runs the gateway backend.
    pub async fn run(self) -> Result<(), Error> {
        let token = self.token;
        let dispatch = self.dispatch;
        let mut shutdown = std::pin::pin!(self.shutdown);

        let shardid = tw::ShardId::ONE;
        let intents = tw::Intents::empty();
        let mut shard = tw::Shard::new(shardid, token, intents);

        let wanted_events = tw::EventTypeFlags::INTERACTION_CREATE;

        let mut closing = false;
        let mut deadline = std::pin::pin!(tokio::time::sleep(Duration::MAX));

        loop {
            tokio::select! { biased;
                _ = &mut shutdown, if !closing => {
                    shard.close(tw::CloseFrame::NORMAL);

                    deadline.as_mut().reset(Instant::now() + Duration::from_secs(5));
                    closing = true;
                }

                _ = &mut deadline, if closing => {
                    dispatch.stop().await;
                    return Err(Error::msg("timed out waiting for gateway to close"));
                }

                event = shard.next_event(wanted_events) => {
                    let event = match event {
                        Some(Ok(event)) => event,
                        Some(Err(_)) => continue,
                        None => {
                            dispatch.stop().await;
                            return Err(Error::msg("gateway fatally closed"));
                        },
                    };

                    let interaction = match event {
                        tw::Event::GatewayClose(_) if closing => break,
                        tw::Event::InteractionCreate(interaction) => interaction.0,
                        _ => continue,
                    };

                    dispatch.send(ResponseSink::new(None), interaction);
                }
            }
        }

        dispatch.stop().await;

        Ok(())
    }
}
