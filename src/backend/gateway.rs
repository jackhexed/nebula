use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use tokio_util::{sync::CancellationToken, task::TaskTracker};
use twilight_gateway::StreamExt;

use crate::{Bot, Error, action::Action, backend::ResponseSink, shared::Shared};

mod tw {
    pub use twilight_gateway::{CloseFrame, Event, EventTypeFlags, Intents, Shard, ShardId};
}

/// A backend that receives actions over the gateway.
///
/// This backend is for local testing, as it needs no public HTTP endpoint. It
/// opens a single shard with no intents and ignores every event except
/// interactions.
///
/// Clones are handles to the same gateway, so a clone can
/// [`stop`](Self::stop) a gateway that another clone started.
#[derive(Clone)]
pub struct Gateway<T>
where
    T: Future<Output = ()> + Send + 'static,
{
    inner: Arc<Inner<T>>,
}

impl<T> Gateway<T>
where
    T: Future<Output = ()> + Send + 'static,
{
    /// Creates a gateway for the given bot.
    ///
    /// The gateway does not connect until [`start`](Self::start) is called.
    pub fn new(bot: Bot<T>) -> Self {
        let inner = Arc::new(Inner::new(bot));

        Self { inner }
    }

    /// Connects to the gateway and handles actions until the gateway is
    /// stopped.
    ///
    /// Each action is handled on its own task. After [`stop`](Self::stop) is
    /// called, this function closes the connection and waits for running
    /// handlers to finish before returning.
    ///
    /// # Errors
    ///
    /// This function will return an error if the gateway has already been
    /// started, even if it has since stopped, or if the connection closes
    /// unexpectedly.
    pub async fn start(&self) -> Result<(), Error> {
        self.inner.start().await
    }

    /// Tells the gateway to stop.
    ///
    /// This function returns immediately. [`start`](Self::start) returns once
    /// shutdown completes.
    pub fn stop(&self) {
        self.inner.stop();
    }
}

/// The state shared by every clone of a [`Gateway`].
struct Inner<T>
where
    T: Future<Output = ()> + Send + 'static,
{
    bot: Bot<T>,
    cancel: CancellationToken,
    tracker: TaskTracker,
    started: AtomicBool,
}

impl<T> Inner<T>
where
    T: Future<Output = ()> + Send + 'static,
{
    fn new(bot: Bot<T>) -> Self {
        let cancel = CancellationToken::new();
        let tracker = TaskTracker::new();
        let started = AtomicBool::new(false);

        Self {
            bot,
            cancel,
            tracker,
            started,
        }
    }

    fn stop(&self) {
        self.cancel.cancel();
        self.tracker.close();
    }

    async fn start(&self) -> Result<(), Error> {
        if self.started.swap(true, Ordering::AcqRel) {
            return Err(Error::new("bot already started"));
        }

        let cancel = self.cancel.clone();
        let tracker = self.tracker.clone();
        let shared = Shared::new(self.bot.token.clone(), self.bot.app_id);
        let handler = self.bot.handler;

        let spawn = |event| {
            let shared = Arc::clone(&shared);

            tracker.spawn(async move {
                if let tw::Event::InteractionCreate(interaction) = event {
                    let sink = ResponseSink::new(None);

                    handler(Action::parse(shared, sink, interaction.0)).await;
                }
            })
        };

        let mut shard = tw::Shard::new(
            tw::ShardId::ONE,
            self.bot.token.clone(),
            tw::Intents::empty(),
        );

        loop {
            tokio::select! { biased;
                _ = cancel.cancelled() => {
                    shard.close(tw::CloseFrame::NORMAL);
                    break;
                }

                event = shard.next_event(tw::EventTypeFlags::INTERACTION_CREATE) => {
                    match event {
                        Some(Ok(event)) => {
                            spawn(event);
                        }

                        Some(Err(..)) => {}
                        None => {
                            return Err(Error::new("gateway closed unexpectedly"));
                        }
                    }
                }
            }
        }

        loop {
            match shard
                .next_event(tw::EventTypeFlags::INTERACTION_CREATE)
                .await
            {
                Some(Ok(tw::Event::GatewayClose(..))) => break,
                Some(Ok(event)) => {
                    spawn(event);
                }

                Some(Err(..)) => {}
                None => {
                    return Err(Error::new("gateway closed unexpectedly"));
                }
            }
        }

        tracker.close();
        tracker.wait().await;

        Ok(())
    }
}
