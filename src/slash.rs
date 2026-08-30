mod tw {
    pub use twilight_model::application::command::Command;
    pub use twilight_model::application::command::CommandOption;
    pub use twilight_model::application::command::CommandOptionType;
    pub use twilight_model::application::command::CommandOptionValue;
    pub use twilight_model::application::command::CommandType;
    pub use twilight_model::application::interaction::InteractionContextType;
}

/// A slash command.
///
/// Slash commands can have parameters or subcommands, but not both. A slash
/// command may subcommands with subcommands, but a subcommand may not have
/// subcommands.
pub struct Slash {
    name: String,
    description: String,

    params: Vec<SlashParam>,
    subcommands: Vec<Slash>,
}

impl Slash {
    /// Creates a new slash command.
    ///
    /// Slash command names may only contain lowercase letters, underscores, and
    /// dashes. They must be between 1 and 32 characters long.
    ///
    /// The description must be between 1 and 100 characters long, but has no
    /// restrictions on the characters it may contain.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        let name = name.into();
        let description = description.into();

        Self {
            name,
            description,
            params: Vec::new(),
            subcommands: Vec::new(),
        }
    }

    /// Adds a parameter to the slash command.
    ///
    /// If the command has subcommands, this will panic.
    pub fn param(mut self, param: impl Into<SlashParam>) -> Self {
        assert!(
            self.subcommands.is_empty(),
            "cannot add parameters to a command with subcommands"
        );

        self.params.push(param.into());
        self
    }

    /// Adds a subcommand to the slash command.
    ///
    /// If the command has parameters, or if the subcommand has subcommands with
    /// subcommands, this will panic.
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

impl From<Slash> for tw::Command {
    fn from(value: Slash) -> Self {
        let name = value.name;
        let description = value.description;

        let mut options = Vec::new();

        for param in value.params {
            options.push(param.into());
        }

        for subcommand in value.subcommands {
            options.push(subcommand.into());
        }

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
            version: twilight_model::id::Id::new(1),
        }
    }
}

impl From<Slash> for tw::CommandOption {
    fn from(value: Slash) -> Self {
        let name = value.name;
        let description = value.description;
        let kind = if value.params.is_empty() {
            tw::CommandOptionType::SubCommandGroup
        } else {
            tw::CommandOptionType::SubCommand
        };

        let mut options = Vec::new();

        for param in value.params {
            options.push(param.into());
        }

        for subcommand in value.subcommands {
            options.push(subcommand.into());
        }

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

/// A parameter for a slash command.
pub enum SlashParam {
    Text(SlashParamText),
    Integer(SlashParamInteger),
    Number(SlashParamNumber),
    Boolean(SlashParamBoolean),
    User(SlashParamUser),
    Role(SlashParamRole),
}

impl SlashParam {
    /// Creates a new text parameter.
    pub fn text(name: impl Into<String>, description: impl Into<String>) -> SlashParamText {
        SlashParamText::new(name, description)
    }

    /// Creates a new integer parameter.
    pub fn integer(name: impl Into<String>, description: impl Into<String>) -> SlashParamInteger {
        SlashParamInteger::new(name, description)
    }

    /// Creates a new number parameter.
    pub fn number(name: impl Into<String>, description: impl Into<String>) -> SlashParamNumber {
        SlashParamNumber::new(name, description)
    }

    /// Creates a new boolean parameter.
    pub fn boolean(name: impl Into<String>, description: impl Into<String>) -> SlashParamBoolean {
        SlashParamBoolean::new(name, description)
    }

    /// Creates a new user parameter.
    pub fn user(name: impl Into<String>, description: impl Into<String>) -> SlashParamUser {
        SlashParamUser::new(name, description)
    }

    /// Creates a new role parameter.
    pub fn role(name: impl Into<String>, description: impl Into<String>) -> SlashParamRole {
        SlashParamRole::new(name, description)
    }
}

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
    /// Creates a new text parameter.
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

    /// Sets the parameter as required.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Sets the minimum length of the text input.
    pub fn min_length(mut self, min_length: u16) -> Self {
        self.min_length = Some(min_length);
        self
    }

    /// Sets the maximum length of the text input.
    pub fn max_length(mut self, max_length: u16) -> Self {
        self.max_length = Some(max_length);
        self
    }
}

impl From<SlashParamText> for SlashParam {
    fn from(value: SlashParamText) -> Self {
        Self::Text(value)
    }
}

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
    /// Creates a new integer parameter.
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

    /// Sets the parameter as required.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Sets the minimum value of the integer input.
    pub fn min_value(mut self, min_value: i64) -> Self {
        self.min_value = Some(min_value);
        self
    }

    /// Sets the maximum value of the integer input.
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
    /// Creates a new number parameter.
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

    /// Sets the parameter as required.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Sets the minimum value of the number input.
    pub fn min_value(mut self, min_value: f64) -> Self {
        self.min_value = Some(min_value);
        self
    }

    /// Sets the maximum value of the number input.
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
    /// Creates a new boolean parameter.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        let name = name.into();
        let description = description.into();

        Self {
            name,
            description,
            required: false,
        }
    }

    /// Sets the parameter as required.
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
    /// Creates a new user parameter.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        let name = name.into();
        let description = description.into();

        Self {
            name,
            description,
            required: false,
        }
    }

    /// Sets the parameter as required.
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
    /// Creates a new role parameter.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        let name = name.into();
        let description = description.into();

        Self {
            name,
            description,
            required: false,
        }
    }

    /// Sets the parameter as required.
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
