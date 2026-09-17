//! Nebula takes a somewhat unconventional approach to caching.
//!
//! Most Discord API wrappers connect to the gateway and cache all data
//! received, or a subset of all data received as decided by the user. This is
//! not workable for Nebula, as Nebula is not designed to be connected to the
//! gateway.
//!
//! Nebula must cache data that is retrieved, or the small amount of data that
//! is provided with interactions. However, this introduces a difficult problem:
//! cache invalidation.
//!
//! We assume that all action contexts are short lived, and that makes it easy
//! to attach a cache to each action that caches only data received with that
//! action, or data retrieved within that action's context. All data is
//! retained for the full lifetime of the action context, and then the cache is
//! dropped with the action context.

use std::sync::{Arc, Mutex};

/// A cache for data received with or retrieved during a single action.
pub(crate) struct Cache {
    inner: Mutex<Inner>,
}

impl Cache {
    /// Creates an empty cache.
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: Mutex::new(Inner::default()),
        })
    }
}

#[derive(Default)]
struct Inner {}
