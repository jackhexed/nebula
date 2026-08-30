use std::{collections::HashMap, ops::Deref};

use crate::{
    Role, Snowflake, User,
    action::ActionCtx,
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
    pub fn optional<'a, T>(&'a self, name: &str) -> Option<T>
    where
        T: From<&'a SlashArg>,
    {
        self.args.get(name).map(Into::into)
    }

    /// Returns the value of a required argument.
    ///
    /// If the argument is not present, panics with a message indicating that
    /// the required argument is missing.
    pub fn required<'a, T>(&'a self, name: &str) -> T
    where
        T: From<&'a SlashArg>,
    {
        self.optional(name).expect("required argument is missing")
    }

    /// Parses a slash command action from the given action context and command
    /// data.
    pub fn parse(action_ctx: ActionCtx, data: Box<tw::CommandData>) -> Self {
        let mut path = data.name;
        let mut options = data.options;

        if let Some(first) = options.first() {
            if matches!(first.value, tw::CommandOptionValue::SubCommandGroup(..)) {
                let opt = options.into_iter().next().unwrap();

                path.push(' ');
                path.push_str(&opt.name);

                options = match opt.value {
                    tw::CommandOptionValue::SubCommandGroup(opts) => opts,
                    _ => unreachable!(),
                };

                let opt = options.into_iter().next().unwrap();

                path.push(' ');
                path.push_str(&opt.name);

                options = match opt.value {
                    tw::CommandOptionValue::SubCommandGroup(opts) => opts,
                    _ => unreachable!(),
                };
            } else if matches!(first.value, tw::CommandOptionValue::SubCommand(..)) {
                let opt = options.into_iter().next().unwrap();

                path.push(' ');
                path.push_str(&opt.name);

                options = match opt.value {
                    tw::CommandOptionValue::SubCommand(opts) => opts,
                    _ => unreachable!(),
                };
            }
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

impl<'a> From<&'a SlashArg> for bool {
    fn from(value: &'a SlashArg) -> Self {
        match value {
            SlashArg::Boolean(b) => *b,
            _ => unreachable!("type mismatch: expected boolean"),
        }
    }
}

impl<'a> From<&'a SlashArg> for i64 {
    fn from(value: &'a SlashArg) -> Self {
        match value {
            SlashArg::Integer(i) => *i,
            _ => unreachable!("type mismatch: expected integer"),
        }
    }
}

impl<'a> From<&'a SlashArg> for f64 {
    fn from(value: &'a SlashArg) -> Self {
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

impl<'a> From<&'a SlashArg> for Box<str> {
    fn from(value: &'a SlashArg) -> Self {
        match value {
            SlashArg::String(s) => s.to_owned(),
            _ => unreachable!("type mismatch: expected string"),
        }
    }
}

impl<'a> From<&'a SlashArg> for String {
    fn from(value: &'a SlashArg) -> Self {
        match value {
            SlashArg::String(s) => s.to_string(),
            _ => unreachable!("type mismatch: expected string"),
        }
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

impl<'a> From<&'a SlashArg> for User {
    fn from(value: &'a SlashArg) -> Self {
        match value {
            SlashArg::User(user) => user.clone(),
            _ => unreachable!("type mismatch: expected user"),
        }
    }
}

impl<'a> From<&'a SlashArg> for Snowflake<UserMarker> {
    fn from(value: &'a SlashArg) -> Self {
        match value {
            SlashArg::User(user) => user.id(),
            _ => unreachable!("type mismatch: expected user"),
        }
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

impl<'a> From<&'a SlashArg> for Role {
    fn from(value: &'a SlashArg) -> Self {
        match value {
            SlashArg::Role(role) => role.clone(),
            _ => unreachable!("type mismatch: expected role"),
        }
    }
}

impl<'a> From<&'a SlashArg> for Snowflake<RoleMarker> {
    fn from(value: &'a SlashArg) -> Self {
        match value {
            SlashArg::Role(role) => role.id(),
            _ => unreachable!("type mismatch: expected role"),
        }
    }
}
