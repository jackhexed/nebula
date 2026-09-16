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

/// The context for a slash command action.
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
    /// Returns the path of the slash command, which is a space-separated string
    /// of the command name and any subcommand names.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns a reference to the arguments the slash command was invoked with.
    ///
    /// The keys are the argument names, and the values are the argument values.
    pub fn args(&self) -> &HashMap<Box<str>, SlashArg> {
        &self.args
    }

    /// Returns the value of an optional argument.
    ///
    /// If the argument is not present, returns `None`.
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

/// Represents a slash command argument.
pub enum SlashArg {
    Boolean(bool),
    Integer(i64),
    Number(f64),
    String(Box<str>),

    User(User),
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
            tw::CommandOptionValue::String(str) => Self::String(str.into_boxed_str()),

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
    fn from(value: &SlashArg) -> Self {
        match value {
            SlashArg::Boolean(b) => *b,
            _ => unreachable!("type mismatch: expected boolean"),
        }
    }
}

impl From<&SlashArg> for i64 {
    fn from(value: &SlashArg) -> Self {
        match value {
            SlashArg::Integer(i) => *i,
            _ => unreachable!("type mismatch: expected integer"),
        }
    }
}

impl From<&SlashArg> for f64 {
    fn from(value: &SlashArg) -> Self {
        match value {
            SlashArg::Number(n) => *n,
            _ => unreachable!("type mismatch: expected number"),
        }
    }
}

impl<'a> From<&'a SlashArg> for &'a str {
    fn from(value: &'a SlashArg) -> Self {
        match value {
            SlashArg::String(s) => s,
            _ => unreachable!("type mismatch: expected string"),
        }
    }
}

impl From<&SlashArg> for Box<str> {
    fn from(value: &SlashArg) -> Self {
        <&str>::from(value).into()
    }
}

impl From<&SlashArg> for String {
    fn from(value: &SlashArg) -> Self {
        <&str>::from(value).into()
    }
}

impl<'a> From<&'a SlashArg> for &'a User {
    fn from(value: &'a SlashArg) -> Self {
        match value {
            SlashArg::User(user) => user,
            _ => unreachable!("type mismatch: expected user"),
        }
    }
}

impl From<&SlashArg> for User {
    fn from(value: &SlashArg) -> Self {
        <&User>::from(value).clone()
    }
}

impl From<&SlashArg> for Snowflake<UserMarker> {
    fn from(value: &SlashArg) -> Self {
        <&User>::from(value).id()
    }
}

impl<'a> From<&'a SlashArg> for &'a Role {
    fn from(value: &'a SlashArg) -> Self {
        match value {
            SlashArg::Role(role) => role,
            _ => unreachable!("type mismatch: expected role"),
        }
    }
}

impl From<&SlashArg> for Role {
    fn from(value: &SlashArg) -> Self {
        <&Role>::from(value).clone()
    }
}

impl From<&SlashArg> for Snowflake<RoleMarker> {
    fn from(value: &SlashArg) -> Self {
        <&Role>::from(value).id()
    }
}
