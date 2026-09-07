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

/// A message the bot displays.
///
/// A message is a collection of up to 40 items. See [`MsgItem`] for what a
/// message can hold. A message may be ephemeral, meaning it is only visible to
/// the user who triggered the interaction. Ephemeral messages are not persisted
/// and cannot be edited once the interaction token expires.
///
/// A [`Msg`] can be created with the [`msg!`] macro, or by using the builder
/// methods directly.
///
/// You can also use the [`From`] implementations to convert a single item into
/// a [`Msg`]. For example, a [`Frame`] can be used instead of a [`Msg`] in all
/// functions that accept `impl Into<Msg>`.
#[derive(Debug, Default)]
pub struct Msg {
    items: Vec<MsgItem>,
    ephemeral: bool,
}

impl Msg {
    /// Creates an empty, non-ephemeral message.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            ephemeral: false,
        }
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

    /// Sets whether the message is visible only to the user who triggered the
    /// interaction.
    ///
    /// Messages are visible to everyone by default. An ephemeral message cannot
    /// be seen by anyone else, is not persisted, and cannot be edited once the
    /// interaction token expires.
    pub fn ephemeral(mut self, ephemeral: bool) -> Self {
        self.ephemeral = ephemeral;
        self
    }
}

impl From<Frame> for Msg {
    /// Converts a [`Frame`] into a [`Msg`].
    fn from(value: Frame) -> Self {
        Self {
            items: vec![MsgItem::Frame(value)],
            ephemeral: false,
        }
    }
}

impl From<Row> for Msg {
    /// Converts a [`Row`] into a [`Msg`].
    fn from(value: Row) -> Self {
        Self {
            items: vec![MsgItem::Row(value)],
            ephemeral: false,
        }
    }
}

impl From<Button> for Msg {
    /// Converts a [`Button`] into a [`Msg`].
    ///
    /// A button is wrapped in a [`Row`] because a message cannot hold a button
    /// directly.
    fn from(value: Button) -> Self {
        Self {
            items: vec![MsgItem::Row(Row::from(value))],
            ephemeral: false,
        }
    }
}

impl From<[Button; 1]> for Msg {
    /// Converts an array of one [`Button`] into a [`Msg`].
    fn from(value: [Button; 1]) -> Self {
        Self {
            items: vec![MsgItem::Row(Row::from(value))],
            ephemeral: false,
        }
    }
}

impl From<[Button; 2]> for Msg {
    /// Converts an array of two [`Button`]s into a [`Msg`].
    fn from(value: [Button; 2]) -> Self {
        Self {
            items: vec![MsgItem::Row(Row::from(value))],
            ephemeral: false,
        }
    }
}

impl From<[Button; 3]> for Msg {
    /// Converts an array of three [`Button`]s into a [`Msg`].
    fn from(value: [Button; 3]) -> Self {
        Self {
            items: vec![MsgItem::Row(Row::from(value))],
            ephemeral: false,
        }
    }
}

impl From<[Button; 4]> for Msg {
    /// Converts an array of four [`Button`]s into a [`Msg`].
    fn from(value: [Button; 4]) -> Self {
        Self {
            items: vec![MsgItem::Row(Row::from(value))],
            ephemeral: false,
        }
    }
}

impl From<[Button; 5]> for Msg {
    /// Converts an array of five [`Button`]s into a [`Msg`].
    fn from(value: [Button; 5]) -> Self {
        Self {
            items: vec![MsgItem::Row(Row::from(value))],
            ephemeral: false,
        }
    }
}

impl From<Text> for Msg {
    /// Converts a [`Text`] into a [`Msg`].
    fn from(value: Text) -> Self {
        Self {
            items: vec![MsgItem::Text(value)],
            ephemeral: false,
        }
    }
}

impl From<String> for Msg {
    /// Converts a [`String`] into a [`Msg`].
    ///
    /// This conversion wraps the string in a [`Text`], as a message cannot hold
    /// a string directly.
    fn from(value: String) -> Self {
        Self {
            items: vec![MsgItem::Text(value.into())],
            ephemeral: false,
        }
    }
}

impl From<&str> for Msg {
    /// Converts a `&str` into a [`Msg`].
    ///
    /// This conversion wraps the string in a [`Text`], as a message cannot hold
    /// a string directly.
    fn from(value: &str) -> Self {
        Self {
            items: vec![MsgItem::Text(value.into())],
            ephemeral: false,
        }
    }
}

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
///     * [`Button`](crate::item::Button)
///     * `[Button; 1..=5]`
/// * [`Text`]
///     * [`String`]
///     * `&str`
#[derive(Debug)]
pub enum MsgItem {
    Frame(Frame),
    Row(Row),
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
    /// Converts a [`Button`] into a [`MsgItem`].
    ///
    /// A button is wrapped in a [`Row`] because a message cannot hold a button
    /// directly.
    fn from(value: Button) -> Self {
        Self::Row(Row::from(value))
    }
}

impl From<[Button; 1]> for MsgItem {
    /// Converts an array of one [`Button`] into a [`MsgItem`].
    fn from(value: [Button; 1]) -> Self {
        Self::Row(Row::from(value))
    }
}

impl From<[Button; 2]> for MsgItem {
    /// Converts an array of two [`Button`]s into a [`MsgItem`].
    fn from(value: [Button; 2]) -> Self {
        Self::Row(Row::from(value))
    }
}

impl From<[Button; 3]> for MsgItem {
    /// Converts an array of three [`Button`]s into a [`MsgItem`].
    fn from(value: [Button; 3]) -> Self {
        Self::Row(Row::from(value))
    }
}

impl From<[Button; 4]> for MsgItem {
    /// Converts an array of four [`Button`]s into a [`MsgItem`].
    fn from(value: [Button; 4]) -> Self {
        Self::Row(Row::from(value))
    }
}

impl From<[Button; 5]> for MsgItem {
    /// Converts an array of five [`Button`]s into a [`MsgItem`].
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
    /// Converts a [`String`] into a [`MsgItem`].
    ///
    /// This conversion wraps the string in a [`Text`], as a message cannot hold
    /// a string directly.
    fn from(value: String) -> Self {
        Self::Text(value.into())
    }
}

impl From<&str> for MsgItem {
    /// Converts a `&str` into a [`MsgItem`].
    ///
    /// This conversion wraps the string in a [`Text`], as a message cannot hold
    /// a string directly.
    fn from(value: &str) -> Self {
        Self::Text(value.into())
    }
}

impl From<MsgItem> for tw::Component {
    fn from(value: MsgItem) -> Self {
        match value {
            MsgItem::Frame(frame) => Self::Container(frame.into()),
            MsgItem::Row(row) => Self::ActionRow(row.into()),
            MsgItem::Text(text) => Self::TextDisplay(text.into()),
        }
    }
}
