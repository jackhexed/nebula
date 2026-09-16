use std::{fmt::Display, num::NonZeroU64};

mod tw {
    pub use twilight_model::id::Id;
    pub use twilight_model::id::marker::*;
}

/// A marker trait for the type parameter of [`Snowflake`].
trait Marker {}

/// A type-safe representation of [discord's snowflakes](https://discord.com/developers/docs/reference#snowflakes).
///
/// Snowflakes are unique identifiers used by discord to identify various
/// entities, such as users, guilds, channels, and messages. They are 64-bit
/// unsigned integers that encode the timestamp of their creation, as well as
/// other information such as the worker ID and process ID that generated them.
///
/// The type parameter `T` is a marker type that indicates what kind of entity
/// the snowflake represents, such as a user, guild, or channel.
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

impl<T, U> From<tw::Id<T>> for Snowflake<U>
where
    T: Into<U>,
    U: Marker,
{
    fn from(id: tw::Id<T>) -> Self {
        Self::from(id.into_nonzero())
    }
}

impl<T, U> From<&tw::Id<T>> for Snowflake<U>
where
    T: Into<U>,
    U: Marker,
{
    fn from(id: &tw::Id<T>) -> Self {
        Self::from(id.into_nonzero())
    }
}

impl<T, U> From<Snowflake<U>> for tw::Id<T>
where
    T: Into<U>,
    U: Marker,
{
    fn from(snowflake: Snowflake<U>) -> Self {
        Self::from(snowflake.data)
    }
}

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
    /// Creates a new [`Snowflake`] from a `u64` value.
    ///
    /// # Panics
    ///
    /// Snowflakes use a [`NonZeroU64`] internally, so this function will panic if
    /// the provided value is zero.
    pub const fn new(data: u64) -> Self {
        Self {
            data: NonZeroU64::new(data).expect("Snowflake data must be non-zero"),
            mark: std::marker::PhantomData,
        }
    }

    /// Returns the underlying [`NonZeroU64`] value of the snowflake.
    pub fn data(&self) -> NonZeroU64 {
        self.data
    }

    /// Returns the underlying `u64` value of the snowflake.
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
    ($name:ident) => {
        marker!($name, $name);
    };

    ($name:ident, $tw:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name;

        impl Marker for $name {}

        impl From<tw::$tw> for $name {
            fn from(_: tw::$tw) -> Self {
                Self
            }
        }
    };
}

marker!(AppMarker, ApplicationMarker);
marker!(ChannelMarker);
marker!(GuildMarker);
marker!(ActionMarker, InteractionMarker);
marker!(MessageMarker);
marker!(RoleMarker);
marker!(UserMarker);
