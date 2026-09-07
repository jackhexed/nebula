mod tw {
    pub use twilight_model::channel::message::component::Button;
    pub use twilight_model::channel::message::component::ButtonStyle;
}

/// Creates a [`Button`].
///
/// The first argument names the style and gives the button's custom id, or for
/// `link:` its url. `disabled: <bool>` may follow, and a trailing [`format!`]
/// argument list becomes the label.
///
/// ```
/// use nebula::button;
///
/// let count = 3;
///
/// // a primary button with the custom id "submit" and the label "Submit 3 answers"
/// let submit = button!(primary: "submit", "Submit {count} answers");
///
/// // a disabled danger button with the custom id "reset" and the label "Reset"
/// let locked = button!(danger: "reset", disabled: true, "Reset");
///
/// // a link button with the url "https://discord.com" and the label "Open Discord"
/// let docs = button!(link: "https://discord.com", "Open Discord");
/// ```
///
/// Everything after the style is optional, so a button may be built from a
/// style alone. However, a button with no label and no emoji will error when
/// sent.
///
/// ```
/// use nebula::button;
///
/// let bare = button!(secondary: "noop");
/// ```
///
/// # Panics
///
/// This macro uses the [`Button`] builder methods, which panic in a number of
/// cases. See the [`Button`] docs for details.
#[macro_export]
macro_rules! button {
    (primary: $custom_id:expr $(, $($rest:tt)*)?) => {
		$crate::button_go!($crate::item::Button::primary($custom_id) $(, $($rest)*)?)
	};

	(secondary: $custom_id:expr $(, $($rest:tt)*)?) => {
		$crate::button_go!($crate::item::Button::secondary($custom_id) $(, $($rest)*)?)
	};

	(success: $custom_id:expr $(, $($rest:tt)*)?) => {
		$crate::button_go!($crate::item::Button::success($custom_id) $(, $($rest)*)?)
	};

	(danger: $custom_id:expr $(, $($rest:tt)*)?) => {
		$crate::button_go!($crate::item::Button::danger($custom_id) $(, $($rest)*)?)
	};

	(link: $url:expr $(, $($rest:tt)*)?) => {
		$crate::button_go!($crate::item::Button::link($url) $(, $($rest)*)?)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! button_go {
	($acc:expr $(,)?) => {
		$acc
	};

	($acc:expr, disabled: $disabled:expr $(, $($rest:tt)*)?) => {
		$crate::button_go!($acc.disabled($disabled) $(, $($rest)*)?)
	};

	($acc:expr, $fmt:literal $(, $($args:tt)*)?) => {
		$acc.label(format!($fmt $(, $($args)*)?))
	};

	($acc:expr, $($rest:tt)*) => {
		compile_error!(concat!("unexpected tokens: ", stringify!($($rest)*)));
	}
}

/// A clickable button.
///
/// Buttons are typically created with the [`button!`](crate::button) macro, but
/// some may prefer the builder methods instead.
///
/// Buttons live in a [`Row`](crate::item::Row), which holds up to five of them.
/// A lone `Button` or an array of up to five converts into a `Row`, so a button
/// may be passed anywhere a row is accepted.
///
/// A button either dispatches an action or opens a link. The `primary`,
/// `secondary`, `success`, and `danger` styles take a custom id, which is sent
/// back to the bot when the button is clicked; `link` takes a url instead and
/// is never dispatched.
///
/// [Discord docs](https://docs.discord.com/developers/components/reference#button)
#[derive(Debug)]
pub struct Button {
    custom_id: Option<String>,
    url: Option<String>,
    label: Option<String>,
    style: tw::ButtonStyle,
    disabled: bool,
}

impl Button {
    /// Creates a button of the given style dispatching the given custom id.
    ///
    /// # Panics
    ///
    /// This function will panic if the custom id is not between 1 and 100
    /// characters long.
    fn build(style: tw::ButtonStyle, custom_id: impl Into<String>) -> Self {
        let custom_id = custom_id.into();

        assert!(
            (1..=100).contains(&custom_id.chars().count()),
            "button custom id must be between 1 and 100 characters long"
        );

        Self {
            custom_id: Some(custom_id),
            url: None,
            label: None,
            style,
            disabled: false,
        }
    }

    /// Creates a blurple button dispatching the given custom id.
    ///
    /// # Panics
    ///
    /// This function will panic if the custom id is not between 1 and 100
    /// characters long.
    pub fn primary(custom_id: impl Into<String>) -> Self {
        Self::build(tw::ButtonStyle::Primary, custom_id)
    }

    /// Creates a grey button dispatching the given custom id.
    ///
    /// # Panics
    ///
    /// This function will panic if the custom id is not between 1 and 100
    /// characters long.
    pub fn secondary(custom_id: impl Into<String>) -> Self {
        Self::build(tw::ButtonStyle::Secondary, custom_id)
    }

    /// Creates a green button dispatching the given custom id.
    ///
    /// # Panics
    ///
    /// This function will panic if the custom id is not between 1 and 100
    /// characters long.
    pub fn success(custom_id: impl Into<String>) -> Self {
        Self::build(tw::ButtonStyle::Success, custom_id)
    }

    /// Creates a red button dispatching the given custom id.
    ///
    /// # Panics
    ///
    /// This function will panic if the custom id is not between 1 and 100
    /// characters long.
    pub fn danger(custom_id: impl Into<String>) -> Self {
        Self::build(tw::ButtonStyle::Danger, custom_id)
    }

    /// Creates a grey button opening the given url.
    ///
    /// Link buttons have no custom id and never dispatch an action, so the bot
    /// is not notified when one is clicked.
    ///
    /// # Panics
    ///
    /// This function will panic if the url is not between 1 and 512 characters
    /// long.
    pub fn link(url: impl Into<String>) -> Self {
        let url = url.into();

        assert!(
            (1..=512).contains(&url.chars().count()),
            "button url must be between 1 and 512 characters long"
        );

        Self {
            custom_id: None,
            url: Some(url),
            label: None,
            style: tw::ButtonStyle::Link,
            disabled: false,
        }
    }

    /// Sets the text displayed on the button.
    ///
    /// A button with no label and no emoji will error when sent, so a label is
    /// usually required.
    ///
    /// # Panics
    ///
    /// This function will panic if the label is not between 1 and 80 characters
    /// long.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        let label = label.into();

        assert!(
            (1..=80).contains(&label.chars().count()),
            "button label must be between 1 and 80 characters long"
        );

        self.label = Some(label);
        self
    }

    /// Sets whether the button is greyed out and cannot be clicked.
    ///
    /// Buttons are enabled by default.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl From<Button> for tw::Button {
    /// Converts a [`Button`] into a [`twilight_model::channel::message::component::Button`].
    ///
    /// # Panics
    ///
    /// This function will panic if the button has no label and no emoji.
    fn from(value: Button) -> Self {
        assert!(
            value.label.is_some(),
            "button must have a label or an emoji"
        );

        Self {
            id: None,
            custom_id: value.custom_id,
            disabled: value.disabled,
            emoji: None,
            label: value.label,
            style: value.style,
            url: value.url,
            sku_id: None,
        }
    }
}
