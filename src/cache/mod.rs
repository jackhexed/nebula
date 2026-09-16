//! Nebula takes a somewhat unconventional approach to caching.
//!
//! Most discord api wrappers connect to the gateway and cache all data
//! received, or a subset of all data received as decided by the user. This is
//! not workable for nebula, as nebula is not designed to be connected to the
//! gateway.
//!
//! Nebula must cache data that is retrieved, or the small amount of data that
//! is provided with interactions. However, this introduces a difficult problem:
//! cache invalidation.
//!
//! We assume that all interaction contexts are short lived, and that makes it
//! easy to attach a cache to each interaction that caches only data received
//! with that interaction, or data retrieved within that interaction context.
//! All data is retained for the full lifetime of the interaction context, and
//! then the cache is dropped with the interaction context.

use std::sync::{Arc, Mutex};

/// A cache for storing data retrieved from the discord api.
pub(crate) struct Cache {
    inner: Mutex<Inner>,
}

impl Cache {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: Mutex::new(Inner::default()),
        })
    }
}

#[derive(Default)]
struct Inner {}
