//! Contains types for declaring slash commands.
//!
//! A [`Slash`] declares a command's name, description, and parameters or
//! subcommands. When a user runs the command, the bot receives an
//! [`Action::Slash`](crate::action::Action::Slash) holding the arguments for
//! those parameters.

mod tw {
    pub use twilight_model::application::command::Command;
    pub use twilight_model::application::command::CommandOption;
    pub use twilight_model::application::command::CommandOptionType;
    pub use twilight_model::application::command::CommandOptionValue;
    pub use twilight_model::application::command::CommandType;
    pub use twilight_model::application::interaction::InteractionContextType;
    pub use twilight_model::id::Id;
}

/// A slash command.
///
/// A command has either parameters or subcommands, never both. Subcommands may
/// be nested only two levels deep (command -> group -> subcommand), so a
/// subcommand at the deepest level may only have parameters.
///
/// Commands are only available in guilds for now
/// ([#1](https://github.com/jackhexed/nebula/issues/1)).
///
/// [Discord docs](https://docs.discord.com/developers/interactions/application-commands#slash-commands)
pub struct Slash {
    name: String,
    description: String,

    params: Vec<SlashParam>,
    subcommands: Vec<Slash>,
}

impl Slash {
    /// Creates a slash command, or a subcommand when passed to
    /// [`Slash::subcommand`].
    ///
    /// Besides the length limits below, the name must follow Discord's
    /// [naming rules](https://docs.discord.com/developers/interactions/application-commands#application-command-object-application-command-naming).
    /// The description may contain any characters.
    ///
    /// # Panics
    ///
    /// This function will panic if the name is not between 1 and 32 characters
    /// long, or if the description is not between 1 and 100 characters long.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        let name = name.into();
        let description = description.into();

        assert!(
            (1..=32).contains(&name.chars().count()),
            "command name must be between 1 and 32 characters long"
        );

        assert!(
            (1..=100).contains(&description.chars().count()),
            "command description must be between 1 and 100 characters long"
        );

        Self {
            name,
            description,
            params: Vec::new(),
            subcommands: Vec::new(),
        }
    }

    /// Adds a parameter to the command.
    ///
    /// See [`SlashParam`] for the kinds of parameter. Discord requires every
    /// required parameter to come before every optional one.
    ///
    /// # Panics
    ///
    /// This function will panic if the command has subcommands.
    pub fn param(mut self, param: impl Into<SlashParam>) -> Self {
        assert!(
            self.subcommands.is_empty(),
            "cannot add parameters to a command with subcommands"
        );

        self.params.push(param.into());
        self
    }

    /// Adds a subcommand to the command.
    ///
    /// A subcommand that has subcommands of its own is a
    /// [subcommand group](https://docs.discord.com/developers/interactions/application-commands#subcommands-and-subcommand-groups).
    ///
    /// # Panics
    ///
    /// This function will panic if the command has parameters, or if
    /// `subcommand` would nest subcommands more than two levels deep.
    pub fn subcommand(mut self, subcommand: Slash) -> Self {
        assert!(
            self.params.is_empty(),
            "cannot add subcommand to a command with parameters"
        );

        assert!(
            !subcommand
                .subcommands
                .iter()
                .any(|sc| !sc.subcommands.is_empty()),
            "cannot add subcommand with subcommands as a subcommand"
        );

        self.subcommands.push(subcommand);
        self
    }
}

#[doc(hidden)]
impl From<Slash> for tw::Command {
    // TODO: allow non-guild commands (https://github.com/jackhexed/nebula/issues/1)
    fn from(value: Slash) -> Self {
        let name = value.name;
        let description = value.description;

        let options = value
            .params
            .into_iter()
            .map(Into::into)
            .chain(value.subcommands.into_iter().map(Into::into))
            .collect();

        Self {
            application_id: None,
            contexts: Some(vec![tw::InteractionContextType::Guild]),
            default_member_permissions: None,
            #[allow(deprecated)]
            dm_permission: None,
            description,
            description_localizations: None,
            guild_id: None,
            id: None,
            integration_types: None,
            kind: tw::CommandType::ChatInput,
            name,
            name_localizations: None,
            nsfw: None,
            options,
            version: tw::Id::new(1),
        }
    }
}

#[doc(hidden)]
impl From<Slash> for tw::CommandOption {
    fn from(value: Slash) -> Self {
        let name = value.name;
        let description = value.description;
        let kind = if value.subcommands.is_empty() {
            tw::CommandOptionType::SubCommand
        } else {
            tw::CommandOptionType::SubCommandGroup
        };

        let options = value
            .params
            .into_iter()
            .map(Into::into)
            .chain(value.subcommands.into_iter().map(Into::into))
            .collect();

        Self {
            autocomplete: None,
            channel_types: None,
            choices: None,
            description,
            description_localizations: None,
            kind,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name,
            name_localizations: None,
            options: Some(options),
            required: None,
        }
    }
}

/// A slash command parameter.
///
/// This is rarely named directly. Parameters are usually created with the
/// shorthand constructors on this type, such as [`SlashParam::text`], and
/// passed to [`Slash::param`].
///
/// Parameter names and descriptions have the same limits as [`Slash::new`],
/// but are not yet checked.
///
/// [Discord docs](https://docs.discord.com/developers/interactions/application-commands#application-command-object-application-command-option-type)
pub enum SlashParam {
    /// A [`SlashParamText`].
    Text(SlashParamText),

    /// A [`SlashParamInteger`].
    Integer(SlashParamInteger),

    /// A [`SlashParamNumber`].
    Number(SlashParamNumber),

    /// A [`SlashParamBoolean`].
    Boolean(SlashParamBoolean),

    /// A [`SlashParamUser`].
    User(SlashParamUser),

    /// A [`SlashParamRole`].
    Role(SlashParamRole),
}

impl SlashParam {
    /// Alias for [`SlashParamText::new`].
    pub fn text(name: impl Into<String>, description: impl Into<String>) -> SlashParamText {
        SlashParamText::new(name, description)
    }

    /// Alias for [`SlashParamInteger::new`].
    pub fn integer(name: impl Into<String>, description: impl Into<String>) -> SlashParamInteger {
        SlashParamInteger::new(name, description)
    }

    /// Alias for [`SlashParamNumber::new`].
    pub fn number(name: impl Into<String>, description: impl Into<String>) -> SlashParamNumber {
        SlashParamNumber::new(name, description)
    }

    /// Alias for [`SlashParamBoolean::new`].
    pub fn boolean(name: impl Into<String>, description: impl Into<String>) -> SlashParamBoolean {
        SlashParamBoolean::new(name, description)
    }

    /// Alias for [`SlashParamUser::new`].
    pub fn user(name: impl Into<String>, description: impl Into<String>) -> SlashParamUser {
        SlashParamUser::new(name, description)
    }

    /// Alias for [`SlashParamRole::new`].
    pub fn role(name: impl Into<String>, description: impl Into<String>) -> SlashParamRole {
        SlashParamRole::new(name, description)
    }
}

#[doc(hidden)]
impl From<SlashParam> for tw::CommandOption {
    fn from(value: SlashParam) -> Self {
        match value {
            SlashParam::Text(param) => param.into(),
            SlashParam::Integer(param) => param.into(),
            SlashParam::Number(param) => param.into(),
            SlashParam::Boolean(param) => param.into(),
            SlashParam::User(param) => param.into(),
            SlashParam::Role(param) => param.into(),
        }
    }
}

/// A text parameter for a slash command.
pub struct SlashParamText {
    name: String,
    description: String,
    required: bool,

    min_length: Option<u16>,
    max_length: Option<u16>,
}

impl SlashParamText {
    /// Creates a text parameter.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        let name = name.into();
        let description = description.into();

        Self {
            name,
            description,
            required: false,
            min_length: None,
            max_length: None,
        }
    }

    /// Makes the parameter required.
    ///
    /// Parameters are optional by default.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Sets the minimum length of the text.
    ///
    /// # Panics
    ///
    /// This function will panic if the minimum length is greater than 6000, or
    /// if it is greater than the maximum length.
    pub fn min_length(mut self, min_length: u16) -> Self {
        assert!(
            min_length <= 6000,
            "minimum length cannot be greater than 6000"
        );

        assert!(
            self.max_length.is_none_or(|max| min_length <= max),
            "minimum length cannot be greater than maximum length"
        );

        self.min_length = Some(min_length);
        self
    }

    /// Sets the maximum length of the text.
    ///
    /// # Panics
    ///
    /// This function will panic if the maximum length is not between 1 and
    /// 6000, or if it is less than the minimum length.
    pub fn max_length(mut self, max_length: u16) -> Self {
        assert!(max_length != 0, "maximum length cannot be 0");

        assert!(
            max_length <= 6000,
            "maximum length cannot be greater than 6000"
        );

        assert!(
            self.min_length.is_none_or(|min| max_length >= min),
            "maximum length cannot be less than minimum length"
        );

        self.max_length = Some(max_length);
        self
    }
}

impl From<SlashParamText> for SlashParam {
    fn from(value: SlashParamText) -> Self {
        Self::Text(value)
    }
}

#[doc(hidden)]
impl From<SlashParamText> for tw::CommandOption {
    fn from(value: SlashParamText) -> Self {
        tw::CommandOption {
            autocomplete: None,
            channel_types: None,
            choices: None,
            description: value.description,
            description_localizations: None,
            kind: tw::CommandOptionType::String,
            max_length: value.max_length,
            max_value: None,
            min_length: value.min_length,
            min_value: None,
            name: value.name,
            name_localizations: None,
            options: None,
            required: if value.required { Some(true) } else { None },
        }
    }
}

/// An integer parameter for a slash command.
pub struct SlashParamInteger {
    name: String,
    description: String,
    required: bool,

    min_value: Option<i64>,
    max_value: Option<i64>,
}

impl SlashParamInteger {
    /// Creates an integer parameter.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        let name = name.into();
        let description = description.into();

        Self {
            name,
            description,
            required: false,
            min_value: None,
            max_value: None,
        }
    }

    /// Makes the parameter required.
    ///
    /// Parameters are optional by default.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Sets the minimum value of the integer.
    pub fn min_value(mut self, min_value: i64) -> Self {
        self.min_value = Some(min_value);
        self
    }

    /// Sets the maximum value of the integer.
    pub fn max_value(mut self, max_value: i64) -> Self {
        self.max_value = Some(max_value);
        self
    }
}

impl From<SlashParamInteger> for SlashParam {
    fn from(value: SlashParamInteger) -> Self {
        Self::Integer(value)
    }
}

#[doc(hidden)]
impl From<SlashParamInteger> for tw::CommandOption {
    fn from(value: SlashParamInteger) -> Self {
        tw::CommandOption {
            autocomplete: None,
            channel_types: None,
            choices: None,
            description: value.description,
            description_localizations: None,
            kind: tw::CommandOptionType::Integer,
            max_length: None,
            max_value: value.max_value.map(tw::CommandOptionValue::Integer),
            min_length: None,
            min_value: value.min_value.map(tw::CommandOptionValue::Integer),
            name: value.name,
            name_localizations: None,
            options: None,
            required: if value.required { Some(true) } else { None },
        }
    }
}

/// A number parameter for a slash command.
pub struct SlashParamNumber {
    name: String,
    description: String,
    required: bool,

    min_value: Option<f64>,
    max_value: Option<f64>,
}

impl SlashParamNumber {
    /// Creates a number parameter.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        let name = name.into();
        let description = description.into();

        Self {
            name,
            description,
            required: false,
            min_value: None,
            max_value: None,
        }
    }

    /// Makes the parameter required.
    ///
    /// Parameters are optional by default.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Sets the minimum value of the number.
    pub fn min_value(mut self, min_value: f64) -> Self {
        self.min_value = Some(min_value);
        self
    }

    /// Sets the maximum value of the number.
    pub fn max_value(mut self, max_value: f64) -> Self {
        self.max_value = Some(max_value);
        self
    }
}

impl From<SlashParamNumber> for SlashParam {
    fn from(value: SlashParamNumber) -> Self {
        Self::Number(value)
    }
}

#[doc(hidden)]
impl From<SlashParamNumber> for tw::CommandOption {
    fn from(value: SlashParamNumber) -> Self {
        tw::CommandOption {
            autocomplete: None,
            channel_types: None,
            choices: None,
            description: value.description,
            description_localizations: None,
            kind: tw::CommandOptionType::Number,
            max_length: None,
            max_value: value.max_value.map(tw::CommandOptionValue::Number),
            min_length: None,
            min_value: value.min_value.map(tw::CommandOptionValue::Number),
            name: value.name,
            name_localizations: None,
            options: None,
            required: if value.required { Some(true) } else { None },
        }
    }
}

/// A boolean parameter for a slash command.
pub struct SlashParamBoolean {
    name: String,
    description: String,
    required: bool,
}

impl SlashParamBoolean {
    /// Creates a boolean parameter.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        let name = name.into();
        let description = description.into();

        Self {
            name,
            description,
            required: false,
        }
    }

    /// Makes the parameter required.
    ///
    /// Parameters are optional by default.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

impl From<SlashParamBoolean> for SlashParam {
    fn from(value: SlashParamBoolean) -> Self {
        Self::Boolean(value)
    }
}

#[doc(hidden)]
impl From<SlashParamBoolean> for tw::CommandOption {
    fn from(value: SlashParamBoolean) -> Self {
        tw::CommandOption {
            autocomplete: None,
            channel_types: None,
            choices: None,
            description: value.description,
            description_localizations: None,
            kind: tw::CommandOptionType::Boolean,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name: value.name,
            name_localizations: None,
            options: None,
            required: if value.required { Some(true) } else { None },
        }
    }
}

/// A user parameter for a slash command.
pub struct SlashParamUser {
    name: String,
    description: String,
    required: bool,
}

impl SlashParamUser {
    /// Creates a user parameter.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        let name = name.into();
        let description = description.into();

        Self {
            name,
            description,
            required: false,
        }
    }

    /// Makes the parameter required.
    ///
    /// Parameters are optional by default.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

impl From<SlashParamUser> for SlashParam {
    fn from(value: SlashParamUser) -> Self {
        Self::User(value)
    }
}

#[doc(hidden)]
impl From<SlashParamUser> for tw::CommandOption {
    fn from(value: SlashParamUser) -> Self {
        tw::CommandOption {
            autocomplete: None,
            channel_types: None,
            choices: None,
            description: value.description,
            description_localizations: None,
            kind: tw::CommandOptionType::User,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name: value.name,
            name_localizations: None,
            options: None,
            required: if value.required { Some(true) } else { None },
        }
    }
}

/// A role parameter for a slash command.
pub struct SlashParamRole {
    name: String,
    description: String,
    required: bool,
}

impl SlashParamRole {
    /// Creates a role parameter.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        let name = name.into();
        let description = description.into();

        Self {
            name,
            description,
            required: false,
        }
    }

    /// Makes the parameter required.
    ///
    /// Parameters are optional by default.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

impl From<SlashParamRole> for SlashParam {
    fn from(value: SlashParamRole) -> Self {
        Self::Role(value)
    }
}

#[doc(hidden)]
impl From<SlashParamRole> for tw::CommandOption {
    fn from(value: SlashParamRole) -> Self {
        tw::CommandOption {
            autocomplete: None,
            channel_types: None,
            choices: None,
            description: value.description,
            description_localizations: None,
            kind: tw::CommandOptionType::Role,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name: value.name,
            name_localizations: None,
            options: None,
            required: if value.required { Some(true) } else { None },
        }
    }
}
