//! Contains the [`Snowflake`] ID type and its markers.

use std::{fmt::Display, num::NonZeroU64};

mod tw {
    pub use twilight_model::id::Id;
    pub use twilight_model::id::marker::*;
}

/// A marker trait for the type parameter of [`Snowflake`].
///
/// This trait is private, so downstream crates cannot add their own markers.
trait Marker {}

/// A Discord ID, typed by what it identifies.
///
/// Discord identifies users, guilds, roles, and most other entities with
/// [snowflakes](https://docs.discord.com/developers/reference#snowflakes).
/// The marker type `T`, such as [`UserMarker`], records which kind of entity a
/// snowflake identifies, so a user ID cannot be passed where a guild ID is
/// expected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[allow(private_bounds)]
pub struct Snowflake<T>
where
    T: Marker,
{
    data: NonZeroU64,
    mark: std::marker::PhantomData<T>,
}

impl<T> From<NonZeroU64> for Snowflake<T>
where
    T: Marker,
{
    fn from(value: NonZeroU64) -> Self {
        Self {
            data: value,
            mark: std::marker::PhantomData,
        }
    }
}

impl<T> From<u64> for Snowflake<T>
where
    T: Marker,
{
    /// Converts a `u64` into a [`Snowflake`].
    ///
    /// # Panics
    ///
    /// This function will panic if the value is zero.
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl<T> From<Snowflake<T>> for NonZeroU64
where
    T: Marker,
{
    fn from(snowflake: Snowflake<T>) -> Self {
        snowflake.data
    }
}

impl<T> From<Snowflake<T>> for u64
where
    T: Marker,
{
    fn from(snowflake: Snowflake<T>) -> Self {
        snowflake.data.get()
    }
}

#[doc(hidden)]
impl<T, U> From<tw::Id<T>> for Snowflake<U>
where
    T: Into<U>,
    U: Marker,
{
    fn from(id: tw::Id<T>) -> Self {
        Self::from(id.into_nonzero())
    }
}

#[doc(hidden)]
impl<T, U> From<&tw::Id<T>> for Snowflake<U>
where
    T: Into<U>,
    U: Marker,
{
    fn from(id: &tw::Id<T>) -> Self {
        Self::from(id.into_nonzero())
    }
}

#[doc(hidden)]
impl<T, U> From<Snowflake<U>> for tw::Id<T>
where
    T: Into<U>,
    U: Marker,
{
    fn from(snowflake: Snowflake<U>) -> Self {
        Self::from(snowflake.data)
    }
}

#[doc(hidden)]
impl<T, U> From<&Snowflake<U>> for tw::Id<T>
where
    T: Into<U>,
    U: Marker,
{
    fn from(snowflake: &Snowflake<U>) -> Self {
        Self::from(snowflake.data)
    }
}

#[allow(private_bounds)]
impl<T> Snowflake<T>
where
    T: Marker,
{
    /// Creates a snowflake from a `u64`.
    ///
    /// # Panics
    ///
    /// This function will panic if the value is zero.
    pub const fn new(data: u64) -> Self {
        Self {
            data: NonZeroU64::new(data).expect("Snowflake data must be non-zero"),
            mark: std::marker::PhantomData,
        }
    }

    /// Returns the snowflake as a [`NonZeroU64`].
    pub fn data(&self) -> NonZeroU64 {
        self.data
    }

    /// Returns the snowflake as a `u64`.
    pub fn get(&self) -> u64 {
        self.data.get()
    }
}

impl<T> Display for Snowflake<T>
where
    T: Marker,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

macro_rules! marker {
    ($(#[$meta:meta])* $name:ident) => {
        marker!($(#[$meta])* $name, $name);
    };

    ($(#[$meta:meta])* $name:ident, $tw:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name;

        impl Marker for $name {}

        #[doc(hidden)]
        impl From<tw::$tw> for $name {
            fn from(_: tw::$tw) -> Self {
                Self
            }
        }
    };
}

marker!(
    /// Marks a [`Snowflake`] as identifying an application.
    AppMarker,
    ApplicationMarker
);

marker!(
    /// Marks a [`Snowflake`] as identifying a channel.
    ChannelMarker
);

marker!(
    /// Marks a [`Snowflake`] as identifying a guild.
    GuildMarker
);

marker!(
    /// Marks a [`Snowflake`] as identifying an [action](crate::action).
    ActionMarker,
    InteractionMarker
);

marker!(
    /// Marks a [`Snowflake`] as identifying a message.
    MessageMarker
);

marker!(
    /// Marks a [`Snowflake`] as identifying a role.
    RoleMarker
);

marker!(
    /// Marks a [`Snowflake`] as identifying a user.
    UserMarker
);
