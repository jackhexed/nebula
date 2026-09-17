use std::{collections::HashMap, ops::Deref};

use crate::{
    Snowflake,
    action::ActionCtx,
    model::{Role, User},
    snowflake::{RoleMarker, UserMarker},
};

mod tw {
    pub use twilight_model::application::interaction::application_command::CommandData;
    pub use twilight_model::application::interaction::application_command::CommandOptionValue;
}

/// The context for a [`Slash`](crate::slash::Slash) command that was run.
pub struct SlashCtx {
    action_ctx: ActionCtx,

    path: Box<str>,
    args: HashMap<Box<str>, SlashArg>,
}

impl Deref for SlashCtx {
    type Target = ActionCtx;

    fn deref(&self) -> &Self::Target {
        &self.action_ctx
    }
}

impl SlashCtx {
    /// Returns the command's path: its name followed by any subcommand group
    /// and subcommand names, separated by spaces, such as `"config set"`.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns the arguments the command was run with, keyed by parameter
    /// name.
    ///
    /// Optional parameters the user left out have no entry. Most handlers use
    /// [`required`](Self::required) and [`optional`](Self::optional) instead.
    pub fn args(&self) -> &HashMap<Box<str>, SlashArg> {
        &self.args
    }

    /// Returns the value of an optional argument, or `None` if the user left it
    /// out.
    ///
    /// See [`SlashArg`] for the types `T` may be.
    ///
    /// # Panics
    ///
    /// This function will panic if the argument is present but is not of type
    /// `T`.
    pub fn optional<'a, T>(&'a self, name: &str) -> Option<T>
    where
        T: From<&'a SlashArg>,
    {
        self.args.get(name).map(Into::into)
    }

    /// Returns the value of a required argument.
    ///
    /// See [`SlashArg`] for the types `T` may be.
    ///
    /// # Panics
    ///
    /// This function will panic if the argument is not present, or if it is not
    /// of type `T`.
    pub fn required<'a, T>(&'a self, name: &str) -> T
    where
        T: From<&'a SlashArg>,
    {
        self.optional(name).expect("required argument is missing")
    }

    /// Parses a slash command action from the given action context and command
    /// data.
    pub(crate) fn parse(action_ctx: ActionCtx, data: Box<tw::CommandData>) -> Self {
        let mut path = data.name;
        let mut options = data.options;

        while let Some(opt) = options.pop_if(|opt| {
            matches!(
                opt.value,
                tw::CommandOptionValue::SubCommand(..)
                    | tw::CommandOptionValue::SubCommandGroup(..)
            )
        }) {
            path.push(' ');
            path.push_str(&opt.name);

            options = match opt.value {
                tw::CommandOptionValue::SubCommand(opts)
                | tw::CommandOptionValue::SubCommandGroup(opts) => opts,
                _ => unreachable!(),
            };
        }

        let path = path.into_boxed_str();
        let args = options
            .into_iter()
            .map(|opt| {
                (
                    opt.name.into_boxed_str(),
                    SlashArg::parse(&action_ctx, opt.value),
                )
            })
            .collect();

        Self {
            action_ctx,
            path,
            args,
        }
    }
}

/// The value of a slash command argument.
///
/// This is rarely matched on directly. [`SlashCtx::required`] and
/// [`SlashCtx::optional`] convert an argument into any of the types below, and
/// panic if the argument is a different variant:
///
/// * [`Boolean`](Self::Boolean): `bool`
/// * [`Integer`](Self::Integer): `i64`
/// * [`Number`](Self::Number): `f64`
/// * [`Text`](Self::Text): `&str`, [`String`], `Box<str>`
/// * [`User`](Self::User): [`User`], `&User`, `Snowflake<UserMarker>`
/// * [`Role`](Self::Role): [`Role`], `&Role`, `Snowflake<RoleMarker>`
pub enum SlashArg {
    /// The value of a [boolean parameter](crate::slash::SlashParamBoolean).
    Boolean(bool),

    /// The value of an [integer parameter](crate::slash::SlashParamInteger).
    Integer(i64),

    /// The value of a [number parameter](crate::slash::SlashParamNumber).
    Number(f64),

    /// The value of a [text parameter](crate::slash::SlashParamText).
    Text(Box<str>),

    /// The value of a [user parameter](crate::slash::SlashParamUser).
    User(User),

    /// The value of a [role parameter](crate::slash::SlashParamRole).
    Role(Role),
}

impl SlashArg {
    /// Parses a slash command argument from the given action context and
    /// command option value.
    fn parse(action_ctx: &ActionCtx, value: tw::CommandOptionValue) -> Self {
        match value {
            tw::CommandOptionValue::Boolean(bool) => Self::Boolean(bool),
            tw::CommandOptionValue::Integer(int) => Self::Integer(int),
            tw::CommandOptionValue::Number(num) => Self::Number(num),
            tw::CommandOptionValue::String(text) => Self::Text(text.into_boxed_str()),

            tw::CommandOptionValue::User(user_id) => {
                let user_id = Snowflake::from(user_id);
                let user = User::new(action_ctx.model_ctx(), user_id);

                Self::User(user)
            }

            tw::CommandOptionValue::Role(role_id) => {
                let role_id = Snowflake::from(role_id);
                let role = Role::new(action_ctx.model_ctx(), action_ctx.guild().id(), role_id);

                Self::Role(role)
            }

            _ => unreachable!(),
        }
    }
}

impl From<&SlashArg> for bool {
    /// Converts a [`SlashArg::Boolean`] into a `bool`.
    ///
    /// # Panics
    ///
    /// This function will panic if the argument is not a
    /// [`SlashArg::Boolean`].
    fn from(value: &SlashArg) -> Self {
        match value {
            SlashArg::Boolean(b) => *b,
            _ => unreachable!("type mismatch: expected boolean"),
        }
    }
}

impl From<&SlashArg> for i64 {
    /// Converts a [`SlashArg::Integer`] into an `i64`.
    ///
    /// # Panics
    ///
    /// This function will panic if the argument is not a
    /// [`SlashArg::Integer`].
    fn from(value: &SlashArg) -> Self {
        match value {
            SlashArg::Integer(i) => *i,
            _ => unreachable!("type mismatch: expected integer"),
        }
    }
}

impl From<&SlashArg> for f64 {
    /// Converts a [`SlashArg::Number`] into an `f64`.
    ///
    /// # Panics
    ///
    /// This function will panic if the argument is not a [`SlashArg::Number`].
    fn from(value: &SlashArg) -> Self {
        match value {
            SlashArg::Number(n) => *n,
            _ => unreachable!("type mismatch: expected number"),
        }
    }
}

impl<'a> From<&'a SlashArg> for &'a str {
    /// Converts a [`SlashArg::Text`] into a `&str`.
    ///
    /// # Panics
    ///
    /// This function will panic if the argument is not a [`SlashArg::Text`].
    fn from(value: &'a SlashArg) -> Self {
        match value {
            SlashArg::Text(text) => text,
            _ => unreachable!("type mismatch: expected text"),
        }
    }
}

impl From<&SlashArg> for Box<str> {
    /// Converts a [`SlashArg::Text`] into a `Box<str>`.
    ///
    /// # Panics
    ///
    /// This function will panic if the argument is not a [`SlashArg::Text`].
    fn from(value: &SlashArg) -> Self {
        <&str>::from(value).into()
    }
}

impl From<&SlashArg> for String {
    /// Converts a [`SlashArg::Text`] into a [`String`].
    ///
    /// # Panics
    ///
    /// This function will panic if the argument is not a [`SlashArg::Text`].
    fn from(value: &SlashArg) -> Self {
        <&str>::from(value).into()
    }
}

impl<'a> From<&'a SlashArg> for &'a User {
    /// Converts a [`SlashArg::User`] into a `&User`.
    ///
    /// # Panics
    ///
    /// This function will panic if the argument is not a [`SlashArg::User`].
    fn from(value: &'a SlashArg) -> Self {
        match value {
            SlashArg::User(user) => user,
            _ => unreachable!("type mismatch: expected user"),
        }
    }
}

impl From<&SlashArg> for User {
    /// Converts a [`SlashArg::User`] into a [`User`].
    ///
    /// # Panics
    ///
    /// This function will panic if the argument is not a [`SlashArg::User`].
    fn from(value: &SlashArg) -> Self {
        <&User>::from(value).clone()
    }
}

impl From<&SlashArg> for Snowflake<UserMarker> {
    /// Converts a [`SlashArg::User`] into the user's snowflake.
    ///
    /// # Panics
    ///
    /// This function will panic if the argument is not a [`SlashArg::User`].
    fn from(value: &SlashArg) -> Self {
        <&User>::from(value).id()
    }
}

impl<'a> From<&'a SlashArg> for &'a Role {
    /// Converts a [`SlashArg::Role`] into a `&Role`.
    ///
    /// # Panics
    ///
    /// This function will panic if the argument is not a [`SlashArg::Role`].
    fn from(value: &'a SlashArg) -> Self {
        match value {
            SlashArg::Role(role) => role,
            _ => unreachable!("type mismatch: expected role"),
        }
    }
}

impl From<&SlashArg> for Role {
    /// Converts a [`SlashArg::Role`] into a [`Role`].
    ///
    /// # Panics
    ///
    /// This function will panic if the argument is not a [`SlashArg::Role`].
    fn from(value: &SlashArg) -> Self {
        <&Role>::from(value).clone()
    }
}

impl From<&SlashArg> for Snowflake<RoleMarker> {
    /// Converts a [`SlashArg::Role`] into the role's snowflake.
    ///
    /// # Panics
    ///
    /// This function will panic if the argument is not a [`SlashArg::Role`].
    fn from(value: &SlashArg) -> Self {
        <&Role>::from(value).id()
    }
}
