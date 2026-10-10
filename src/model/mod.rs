//! Contains model types representing Discord entities.
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
//! a handle to the HTTP client and the action's cache. Data is fetched when
//! first needed and stored in that cache, which is dropped when the action
//! ends.
//!
//! This results in slower data access, but fewer web requests and less memory
//! usage, as well as allowing for a more flexible API.
//!
//! Because a model is only a snowflake and a handle, models are cheap to clone
//! and compare equal when their snowflakes are equal.
mod guild;
mod role;
mod user;

pub use guild::Guild;
pub use role::Role;
pub use user::User;
