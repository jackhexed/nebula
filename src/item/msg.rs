use crate::item::{Button, Frame, Row, Text};

mod tw {
    pub use twilight_model::channel::message::MessageFlags;
    pub use twilight_model::channel::message::component::Component;
    pub use twilight_model::http::interaction::InteractionResponseData;
}

/// Creates a [`Msg`].
///
/// The named option `ephemeral: <bool>` may come first, followed by the items
/// the message holds.
///
/// ```
/// use nebula::{frame, msg};
///
/// let reply = msg!(
///     // named options may come first
///     ephemeral: true,
///
///     // after named options, the message's items follow
///     "Here is what I found:",
///     frame!(color: 0x57f287, "Nothing at all."),
/// );
/// ```
///
/// Each item is anything implementing `Into<MsgItem>`, which includes
/// all valid message items and many convenience types. See [`MsgItem`] for
/// details.
///
/// # Panics
///
/// This macro uses the [`Msg`] builder methods, which panic in a number of
/// cases. See the [`Msg`] docs for details.
#[macro_export]
macro_rules! msg {
    ($($args:tt)*) => {
        $crate::msg_go!($crate::item::Msg::new(), $($args)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! msg_go {
    ($acc:expr $(,)?) => {
        $acc
    };

    ($acc:expr, ephemeral: $ephemeral:expr $(, $($rest:tt)*)?) => {
        $crate::msg_go!($acc.ephemeral($ephemeral) $(, $($rest)*)?)
    };

    ($acc:expr, $($item:expr),+ $(,)?) => {
        $acc $(.item($item))+
    };

    ($acc:expr, $($rest:tt)*) => {
        compile_error!(concat!("unexpected tokens: ", stringify!($($rest)*)));
    }
}

/// A message the bot sends.
///
/// Messages are typically created with the [`msg!`](crate::msg) macro, but some
/// may prefer the builder methods instead.
///
/// A message holds up to 40 items; see [`MsgItem`] for what it accepts. Any
/// single item converts into a message holding it, so a [`Frame`], for
/// example, may be passed anywhere an `impl Into<Msg>` is accepted.
///
/// [Discord docs](https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-response-object-messages)
#[derive(Debug, Default)]
pub struct Msg {
    items: Vec<MsgItem>,
    ephemeral: bool,
}

impl Msg {
    /// Creates an empty, non-ephemeral message.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends an item to the message.
    ///
    /// See [`MsgItem`] for what a message accepts.
    ///
    /// # Panics
    ///
    /// This function will panic if the message already holds 40 items.
    pub fn item(mut self, item: impl Into<MsgItem>) -> Self {
        assert!(
            self.items.len() < 40,
            "a message cannot hold more than 40 items"
        );

        self.items.push(item.into());
        self
    }

    /// Sets whether the message is visible only to the user who performed the
    /// action.
    ///
    /// Messages are visible to everyone by default. An ephemeral message is not
    /// persisted, and cannot be edited once the action's token expires.
    pub fn ephemeral(mut self, ephemeral: bool) -> Self {
        self.ephemeral = ephemeral;
        self
    }
}

impl From<Frame> for Msg {
    /// Converts a [`Frame`] into a [`Msg`] holding it.
    fn from(value: Frame) -> Self {
        Self::new().item(value)
    }
}

impl From<Row> for Msg {
    /// Converts a [`Row`] into a [`Msg`] holding it.
    fn from(value: Row) -> Self {
        Self::new().item(value)
    }
}

impl From<Button> for Msg {
    /// Converts a [`Button`] into a [`Msg`] holding it, wrapped in a [`Row`].
    fn from(value: Button) -> Self {
        Self::new().item(value)
    }
}

impl From<[Button; 1]> for Msg {
    /// Converts an array of one [`Button`] into a [`Msg`] holding it, wrapped
    /// in a [`Row`].
    fn from(value: [Button; 1]) -> Self {
        Self::new().item(value)
    }
}

impl From<[Button; 2]> for Msg {
    /// Converts an array of two [`Button`]s into a [`Msg`] holding them,
    /// wrapped in a [`Row`].
    fn from(value: [Button; 2]) -> Self {
        Self::new().item(value)
    }
}

impl From<[Button; 3]> for Msg {
    /// Converts an array of three [`Button`]s into a [`Msg`] holding them,
    /// wrapped in a [`Row`].
    fn from(value: [Button; 3]) -> Self {
        Self::new().item(value)
    }
}

impl From<[Button; 4]> for Msg {
    /// Converts an array of four [`Button`]s into a [`Msg`] holding them,
    /// wrapped in a [`Row`].
    fn from(value: [Button; 4]) -> Self {
        Self::new().item(value)
    }
}

impl From<[Button; 5]> for Msg {
    /// Converts an array of five [`Button`]s into a [`Msg`] holding them,
    /// wrapped in a [`Row`].
    fn from(value: [Button; 5]) -> Self {
        Self::new().item(value)
    }
}

impl From<Text> for Msg {
    /// Converts a [`Text`] into a [`Msg`] holding it.
    fn from(value: Text) -> Self {
        Self::new().item(value)
    }
}

impl From<String> for Msg {
    /// Converts a [`String`] into a [`Msg`] holding it, wrapped in a [`Text`].
    ///
    /// # Panics
    ///
    /// This function will panic if the string is not between 1 and 4000
    /// characters long.
    fn from(value: String) -> Self {
        Self::new().item(value)
    }
}

impl From<&str> for Msg {
    /// Converts a `&str` into a [`Msg`] holding it, wrapped in a [`Text`].
    ///
    /// # Panics
    ///
    /// This function will panic if the string is not between 1 and 4000
    /// characters long.
    fn from(value: &str) -> Self {
        Self::new().item(value)
    }
}

#[doc(hidden)]
impl From<Msg> for tw::InteractionResponseData {
    fn from(value: Msg) -> Self {
        let flags = if value.ephemeral {
            tw::MessageFlags::IS_COMPONENTS_V2 | tw::MessageFlags::EPHEMERAL
        } else {
            tw::MessageFlags::IS_COMPONENTS_V2
        };

        Self {
            allowed_mentions: None,
            attachments: None,
            choices: None,
            components: Some(value.items.into_iter().map(Into::into).collect()),
            content: None,
            custom_id: None,
            embeds: None,
            flags: Some(flags),
            title: None,
            tts: None,
            poll: None,
        }
    }
}

/// An item a [`Msg`] can hold.
///
/// This is rarely named directly. [`Msg::item`] takes anything converting into
/// a `MsgItem`, which is any valid message item, or a convenience type that is
/// converted into a valid message item. See the list below:
///
/// * [`Frame`]
/// * [`Row`]
///     * [`Button`]
///     * `[Button; 1..=5]`
/// * [`Text`]
///     * [`String`]
///     * `&str`
#[derive(Debug)]
pub enum MsgItem {
    /// A [`Frame`].
    Frame(Frame),

    /// A [`Row`].
    Row(Row),

    /// A [`Text`].
    Text(Text),
}

impl From<Frame> for MsgItem {
    /// Converts a [`Frame`] into a [`MsgItem`].
    fn from(value: Frame) -> Self {
        Self::Frame(value)
    }
}

impl From<Row> for MsgItem {
    /// Converts a [`Row`] into a [`MsgItem`].
    fn from(value: Row) -> Self {
        Self::Row(value)
    }
}

impl From<Button> for MsgItem {
    /// Converts a [`Button`] into a [`MsgItem`] by wrapping it in a [`Row`].
    fn from(value: Button) -> Self {
        Self::Row(Row::from(value))
    }
}

impl From<[Button; 1]> for MsgItem {
    /// Converts an array of one [`Button`] into a [`MsgItem`] by wrapping it in
    /// a [`Row`].
    fn from(value: [Button; 1]) -> Self {
        Self::Row(Row::from(value))
    }
}

impl From<[Button; 2]> for MsgItem {
    /// Converts an array of two [`Button`]s into a [`MsgItem`] by wrapping them
    /// in a [`Row`].
    fn from(value: [Button; 2]) -> Self {
        Self::Row(Row::from(value))
    }
}

impl From<[Button; 3]> for MsgItem {
    /// Converts an array of three [`Button`]s into a [`MsgItem`] by wrapping
    /// them in a [`Row`].
    fn from(value: [Button; 3]) -> Self {
        Self::Row(Row::from(value))
    }
}

impl From<[Button; 4]> for MsgItem {
    /// Converts an array of four [`Button`]s into a [`MsgItem`] by wrapping
    /// them in a [`Row`].
    fn from(value: [Button; 4]) -> Self {
        Self::Row(Row::from(value))
    }
}

impl From<[Button; 5]> for MsgItem {
    /// Converts an array of five [`Button`]s into a [`MsgItem`] by wrapping
    /// them in a [`Row`].
    fn from(value: [Button; 5]) -> Self {
        Self::Row(Row::from(value))
    }
}

impl From<Text> for MsgItem {
    /// Converts a [`Text`] into a [`MsgItem`].
    fn from(value: Text) -> Self {
        Self::Text(value)
    }
}

impl From<String> for MsgItem {
    /// Converts a [`String`] into a [`MsgItem`] by wrapping it in a [`Text`].
    ///
    /// # Panics
    ///
    /// This function will panic if the string is not between 1 and 4000
    /// characters long.
    fn from(value: String) -> Self {
        Self::Text(value.into())
    }
}

impl From<&str> for MsgItem {
    /// Converts a `&str` into a [`MsgItem`] by wrapping it in a [`Text`].
    ///
    /// # Panics
    ///
    /// This function will panic if the string is not between 1 and 4000
    /// characters long.
    fn from(value: &str) -> Self {
        Self::Text(value.into())
    }
}

#[doc(hidden)]
impl From<MsgItem> for tw::Component {
    fn from(value: MsgItem) -> Self {
        match value {
            MsgItem::Frame(frame) => Self::Container(frame.into()),
            MsgItem::Row(row) => Self::ActionRow(row.into()),
            MsgItem::Text(text) => Self::TextDisplay(text.into()),
        }
    }
}
