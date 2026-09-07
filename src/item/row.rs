use crate::item::Button;

mod tw {
    pub use twilight_model::channel::message::component::ActionRow;
    pub use twilight_model::channel::message::component::Component;
}

/// Creates a [`Row`].
///
/// Every argument is a [`Button`], and a row holds one to five of them.
///
/// ```
/// use nebula::{button, row};
///
/// let controls = row!(
///     button!(secondary: "prev", "Previous"),
///     button!(secondary: "next", "Next"),
/// );
/// ```
///
/// Passing no buttons, or more than five, is a compile error. Use [`Row::new`]
/// to build a row whose length is not known until runtime.
#[macro_export]
macro_rules! row {
	($($button:expr),+ $(,)?) => {
		$crate::item::Row::from([$($button),*])
	};
}

/// A horizontal row of up to five [`Button`]s.
///
/// Rows are typically created with the [`row!`](crate::row) macro, but some may
/// prefer the builder methods instead.
///
/// Rows live in a [`Msg`](crate::item::Msg) or a [`Frame`](crate::item::Frame).
/// A lone [`Button`] or an array of up to five converts into a row, so this
/// type only needs to be named when the buttons are collected at runtime.
///
/// [Discord docs](https://docs.discord.com/developers/components/reference#action-row)
#[derive(Debug, Default)]
pub struct Row {
    buttons: Vec<Button>,
}

impl Row {
    /// Creates an empty row.
    ///
    /// A row with no buttons will error when sent, so at least one button must
    /// be added with [`Row::button`].
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
        }
    }

    /// Appends a button to the row.
    ///
    /// # Panics
    ///
    /// This function will panic if the row already holds five buttons.
    pub fn button(mut self, button: Button) -> Self {
        assert!(
            self.buttons.len() < 5,
            "an action row cannot hold more than 5 buttons"
        );

        self.buttons.push(button);
        self
    }
}

impl From<Button> for Row {
    /// Converts a [`Button`] into a [`Row`] holding it.
    fn from(value: Button) -> Self {
        Self {
            buttons: vec![value],
        }
    }
}

impl From<[Button; 1]> for Row {
    /// Converts an array of one [`Button`] into a [`Row`].
    fn from(value: [Button; 1]) -> Self {
        Self {
            buttons: value.into_iter().collect(),
        }
    }
}

impl From<[Button; 2]> for Row {
    /// Converts an array of two [`Button`]s into a [`Row`].
    fn from(value: [Button; 2]) -> Self {
        Self {
            buttons: value.into_iter().collect(),
        }
    }
}

impl From<[Button; 3]> for Row {
    /// Converts an array of three [`Button`]s into a [`Row`].
    fn from(value: [Button; 3]) -> Self {
        Self {
            buttons: value.into_iter().collect(),
        }
    }
}

impl From<[Button; 4]> for Row {
    /// Converts an array of four [`Button`]s into a [`Row`].
    fn from(value: [Button; 4]) -> Self {
        Self {
            buttons: value.into_iter().collect(),
        }
    }
}

impl From<[Button; 5]> for Row {
    /// Converts an array of five [`Button`]s into a [`Row`].
    fn from(value: [Button; 5]) -> Self {
        Self {
            buttons: value.into_iter().collect(),
        }
    }
}

impl From<Row> for tw::ActionRow {
    fn from(value: Row) -> Self {
        Self {
            id: None,
            components: value
                .buttons
                .into_iter()
                .map(Into::into)
                .map(tw::Component::Button)
                .collect(),
        }
    }
}
