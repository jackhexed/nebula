use std::sync::Arc;

use tokio_util::task::TaskTracker;

use crate::{action::Action, backend::ResponseSink, rest::Rest};

mod tw {
    pub use twilight_model::application::interaction::Interaction;
}

/// Dispatches actions to the action callback and tracks the tasks it spawns.
pub(crate) struct Dispatch<T, S>
where
    T: Future<Output = ()> + Send + 'static,
    S: Send + Sync + 'static,
{
    rest: Arc<Rest>,
    state: Arc<S>,
    callback: fn(Action<S>) -> T,
    tasks: TaskTracker,
}

impl<T, S> Dispatch<T, S>
where
    T: Future<Output = ()> + Send + 'static,
    S: Send + Sync + 'static,
{
    /// Creates a new dispatch with the given REST client and action callback.
    pub(crate) fn new(rest: Arc<Rest>, state: Arc<S>, callback: fn(Action<S>) -> T) -> Self {
        let tasks = TaskTracker::new();

        Self {
            rest,
            state,
            callback,
            tasks,
        }
    }

    /// Dispatches an action to the action callback.
    pub(crate) fn send(&self, sink: ResponseSink, interaction: tw::Interaction) {
        let rest = Arc::clone(&self.rest);
        let state = Arc::clone(&self.state);
        let callback = self.callback;

        self.tasks.spawn(async move {
            if let Some(action) = Action::parse(rest, sink, state, interaction) {
                callback(action).await;
            }
        });
    }

    /// Waits for all tasks spawned by the dispatch to complete.
    pub(crate) async fn stop(&self) {
        self.tasks.close();
        self.tasks.wait().await;
    }
}
