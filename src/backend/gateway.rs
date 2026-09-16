use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use tokio_util::{sync::CancellationToken, task::TaskTracker};
use twilight_gateway::StreamExt;

use crate::{
    Error,
    action::{Action, ActionCtx},
    backend::ResponseSink,
    bot::Bot,
    shared::Shared,
};

mod tw {
    pub use twilight_gateway::{CloseFrame, Event, EventTypeFlags, Intents, Shard, ShardId};
}

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
    pub fn new(bot: Bot<T>) -> Self {
        let inner = Arc::new(Inner::new(bot));

        Self { inner }
    }

    pub async fn start(&self) -> Result<(), Error> {
        self.inner.start().await
    }

    pub fn stop(&self) {
        self.inner.stop();
    }
}

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

                    handler(ActionCtx::parse(shared, sink, interaction.0)).await;
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
