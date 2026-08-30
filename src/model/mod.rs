//! Contains model types to represent discord entities.
//!
//! Model types represent entities like guilds, users, and roles. They are used
//! both to fetch data about these entities (like username or avatar), and to
//! execute actions on them (like modifying roles or kicking).
//!
//! # Design
//!
//! Frequently when a user accesses a model type, they do not want all of the
//! data about that entity, or may even not want any data at all. For example,
//! a bot may want to kick a user from a guild, but not care about the user's
//! name, avatar, or other data.
//!
//! To make this pattern efficient, model types store only their snowflake and
//! a [`ModelCtx`]. Data is instead stored in the cache.
//!
//! This results in slower data access, but fewer web requests and less memory
//! usage, as well as allowing for a more flexible API.

use std::sync::Arc;

use crate::{cache::Cache, shared::Shared};

mod guild;
mod role;
mod user;

pub use guild::Guild;
pub use role::Role;
pub use user::User;

/// A context for model types, providing access to shared state and cache.
///
/// Model types use this context to access cache data and to make web requests
/// through the shared state, as well as to create new model entities.
#[derive(Clone)]
pub struct ModelCtx {
    shared: Arc<Shared>,
    cache: Arc<Cache>,
}

impl ModelCtx {
    /// Creates a new model context.
    pub fn new(shared: Arc<Shared>, cache: Arc<Cache>) -> Self {
        Self { shared, cache }
    }

    /// Returns a reference to the shared state.
    pub fn shared(&self) -> &Arc<Shared> {
        &self.shared
    }

    /// Returns a reference to the cache.
    pub fn cache(&self) -> &Arc<Cache> {
        &self.cache
    }
}
