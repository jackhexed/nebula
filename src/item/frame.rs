use crate::item::{Button, Row, Text};

mod tw {
    pub use twilight_model::channel::message::component::Component;
    pub use twilight_model::channel::message::component::Container;
}

/// Creates a [`Frame`].
///
/// The named options `color: <u32>` and `spoiler: <bool>` may come first, in
/// any order, followed by the items the frame holds.
///
/// ```
/// use nebula::{button, frame};
///
/// let leak = frame!(
///     // named options may come first, in any order
///     color: 0xed4245,
///     spoiler: true,
///
///     // after named options, the frame's items follow
///     "The butler did it.",
///     button!(link: "https://example.com", "Read more"),
/// );
/// ```
///
/// Each item is anything implementing `Into<FrameItem>`, which includes
/// all valid frame items and many convenience types. See [`FrameItem`] for
/// details.
///
/// # Panics
///
/// This macro uses the [`Frame`] builder methods, which panic in a number of
/// cases. See the [`Frame`] docs for details.
#[macro_export]
macro_rules! frame {
    ($($args:tt)*) => {
        $crate::frame_go!($crate::item::Frame::new(), $($args)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! frame_go {
    ($acc:expr $(,)?) => {
        $acc
    };

    ($acc:expr, color: $color:expr $(, $($rest:tt)*)?) => {
        $crate::frame_go!($acc.color($color) $(, $($rest)*)?)
    };

    ($acc:expr, spoiler: $spoiler:expr $(, $($rest:tt)*)?) => {
        $crate::frame_go!($acc.spoiler($spoiler) $(, $($rest)*)?)
    };

    ($acc:expr, $($item:expr),+ $(,)?) => {
        $acc $(.item($item))+
    };

    ($acc:expr, $($rest:tt)*) => {
        compile_error!(concat!("unexpected tokens: ", stringify!($($rest)*)));
    }
}

/// A bordered container grouping several items together.
///
/// Frames are typically created with the [`frame!`](crate::frame) macro, but
/// some may prefer the builder methods instead.
///
/// A frame draws a border with an optional accent color around its items, and
/// may be hidden behind a spoiler. It takes the place of an embed under
/// components v2.
///
/// A frame cannot hold a frame. Frames may only appear at the top level of a
/// message.
///
/// [Discord docs](https://docs.discord.com/developers/components/reference#container)
#[derive(Debug, Default)]
pub struct Frame {
    items: Vec<FrameItem>,
    color: Option<u32>,
    spoiler: bool,
}

impl Frame {
    /// Creates an empty frame with no accent color.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends an item to the frame.
    ///
    /// See [`FrameItem`] for what a frame accepts.
    ///
    /// # Panics
    ///
    /// This function will panic if the frame already holds 40 items.
    pub fn item(mut self, item: impl Into<FrameItem>) -> Self {
        assert!(
            self.items.len() < 40,
            "a frame cannot hold more than 40 items"
        );

        self.items.push(item.into());
        self
    }

    /// Sets the accent color drawn along the frame's edge.
    ///
    /// The color is an `0xrrggbb` value. Frames have no accent color by
    /// default.
    ///
    /// # Panics
    ///
    /// This function will panic if the color is greater than `0xffffff`.
    pub fn color(mut self, color: u32) -> Self {
        assert!(color <= 0xffffff, "frame color must be an 0xrrggbb value");

        self.color = Some(color);
        self
    }

    /// Sets whether the frame's contents are hidden behind a spoiler.
    ///
    /// Frames are not spoilered by default.
    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.spoiler = spoiler;
        self
    }
}

#[doc(hidden)]
impl From<Frame> for tw::Container {
    fn from(value: Frame) -> Self {
        Self {
            id: None,
            accent_color: value.color.map(Some),
            spoiler: if value.spoiler { Some(true) } else { None },
            components: value.items.into_iter().map(Into::into).collect(),
        }
    }
}

/// An item a [`Frame`] can hold.
///
/// This is rarely named directly. [`Frame::item`] takes anything converting
/// into a `FrameItem`, which is any valid frame item, or a convenience type
/// that is converted into a valid frame item. See the list below:
///
/// * [`Row`]
///     * [`Button`]
///     * `[Button; 1..=5]`
/// * [`Text`]
///     * [`String`]
///     * `&str`
#[derive(Debug)]
pub enum FrameItem {
    /// A [`Row`].
    Row(Row),

    /// A [`Text`].
    Text(Text),
}

impl From<Row> for FrameItem {
    /// Converts a [`Row`] into a [`FrameItem`].
    fn from(value: Row) -> Self {
        Self::Row(value)
    }
}

impl From<Button> for FrameItem {
    /// Converts a [`Button`] into a [`FrameItem`] by wrapping it in a [`Row`].
    fn from(value: Button) -> Self {
        Self::Row(Row::from(value))
    }
}

impl From<[Button; 1]> for FrameItem {
    /// Converts an array of one [`Button`] into a [`FrameItem`] by wrapping it
    /// in a [`Row`].
    fn from(value: [Button; 1]) -> Self {
        Self::Row(Row::from(value))
    }
}

impl From<[Button; 2]> for FrameItem {
    /// Converts an array of two [`Button`]s into a [`FrameItem`] by wrapping
    /// them in a [`Row`].
    fn from(value: [Button; 2]) -> Self {
        Self::Row(Row::from(value))
    }
}

impl From<[Button; 3]> for FrameItem {
    /// Converts an array of three [`Button`]s into a [`FrameItem`] by wrapping
    /// them in a [`Row`].
    fn from(value: [Button; 3]) -> Self {
        Self::Row(Row::from(value))
    }
}

impl From<[Button; 4]> for FrameItem {
    /// Converts an array of four [`Button`]s into a [`FrameItem`] by wrapping
    /// them in a [`Row`].
    fn from(value: [Button; 4]) -> Self {
        Self::Row(Row::from(value))
    }
}

impl From<[Button; 5]> for FrameItem {
    /// Converts an array of five [`Button`]s into a [`FrameItem`] by wrapping
    /// them in a [`Row`].
    fn from(value: [Button; 5]) -> Self {
        Self::Row(Row::from(value))
    }
}

impl From<Text> for FrameItem {
    /// Converts a [`Text`] into a [`FrameItem`].
    fn from(value: Text) -> Self {
        Self::Text(value)
    }
}

impl From<String> for FrameItem {
    /// Converts a [`String`] into a [`FrameItem`] by wrapping it in a [`Text`].
    ///
    /// # Panics
    ///
    /// This function will panic if the string is not between 1 and 4000
    /// characters long.
    fn from(value: String) -> Self {
        Self::Text(value.into())
    }
}

impl From<&str> for FrameItem {
    /// Converts a `&str` into a [`FrameItem`] by wrapping it in a [`Text`].
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
impl From<FrameItem> for tw::Component {
    fn from(value: FrameItem) -> Self {
        match value {
            FrameItem::Text(text) => Self::TextDisplay(text.into()),
            FrameItem::Row(row) => Self::ActionRow(row.into()),
        }
    }
}
