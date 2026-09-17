mod tw {
    pub use twilight_model::channel::message::component::TextDisplay;
}

/// Creates a [`Text`].
///
/// The arguments are a [`format!`] argument list, which becomes the text's
/// markdown content.
///
/// ```
/// use nebula::text;
///
/// let name = "world";
///
/// // a text item with the content "Hello, **world**!"
/// let greeting = text!("Hello, **{name}**!");
/// ```
///
/// # Panics
///
/// This macro uses [`Text::new`], which panics if the content is not between 1
/// and 4000 characters long.
#[macro_export]
macro_rules! text {
    ($($args:tt)*) => {
        $crate::item::Text::new(format!($($args)*))
    };
}

/// A block of markdown text.
///
/// Text is typically created with the [`text!`](crate::text) macro, but some
/// may prefer [`Text::new`] instead.
///
/// Text lives in a [`Msg`](crate::item::Msg) or a [`Frame`](crate::item::Frame).
/// A [`String`] or `&str` converts into a text, so this type only needs to be
/// named when the content is built separately.
///
/// The content is rendered as markdown and may contain mentions.
///
/// [Discord docs](https://docs.discord.com/developers/components/reference#text-display)
#[derive(Debug)]
pub struct Text {
    content: String,
}

impl Text {
    /// Creates a text item with the given markdown content.
    ///
    /// # Panics
    ///
    /// This function will panic if the content is not between 1 and 4000
    /// characters long.
    pub fn new(content: String) -> Self {
        assert!(
            (1..=4000).contains(&content.chars().count()),
            "text content must be between 1 and 4000 characters long"
        );

        Self { content }
    }
}

impl From<String> for Text {
    /// Converts a [`String`] into a [`Text`].
    ///
    /// # Panics
    ///
    /// This function will panic if the string is not between 1 and 4000
    /// characters long.
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for Text {
    /// Converts a `&str` into a [`Text`].
    ///
    /// # Panics
    ///
    /// This function will panic if the string is not between 1 and 4000
    /// characters long.
    fn from(value: &str) -> Self {
        Self::new(value.to_string())
    }
}

#[doc(hidden)]
impl From<Text> for tw::TextDisplay {
    fn from(value: Text) -> Self {
        Self {
            id: None,
            content: value.content,
        }
    }
}
