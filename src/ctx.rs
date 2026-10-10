use std::sync::Arc;

use crate::{cache::Cache, rest::Rest};

/// A context that holds the REST API client and cache.
#[derive(Clone)]
pub struct Ctx {
    rest: Arc<Rest>,
    cache: Arc<Cache>,
}

impl Ctx {
    /// Creates a new context with the given REST API client.
    ///
    /// This will create a new cache for the context.
    pub(crate) fn new(rest: Arc<Rest>) -> Self {
        let cache = Cache::new();

        Self { rest, cache }
    }

    /// Returns the REST API client.
    pub(crate) fn rest(&self) -> &Arc<Rest> {
        &self.rest
    }

    /// Returns the cache.
    pub(crate) fn cache(&self) -> &Arc<Cache> {
        &self.cache
    }
}

impl From<&Ctx> for Ctx {
    fn from(ctx: &Ctx) -> Self {
        ctx.clone()
    }
}
