//! Contains item types used to build action responses.
//!
//! Items map onto Discord's components v2. Every item is an owned builder:
//! methods take `self` and return `Self`, so a tree is built in a single
//! expression without intermediate bindings.
//!
//! Nesting is enforced by the type system rather than at runtime. [`MsgItem`]
//! and [`FrameItem`] enumerate what may appear at each level, and the [`From`]
//! impls on them define the conversions a caller gets for free. Anything a
//! level does not accept simply does not convert, so an invalid tree is a
//! compile error instead of a rejected request.
//!
//! Those [`From`] impls also let a caller skip levels that carry no
//! information. A `&str` becomes a [`Text`], a [`Button`] becomes a [`Row`]
//! holding it, and an array of up to five [`Button`]s becomes a [`Row`] of
//! those buttons.
//!
//! Builder methods that append a child ([`Msg::item`], [`Frame::item`],
//! [`Row::button`]) also accept an `Option` of it and skip `None`. Without
//! this, including an item conditionally would mean breaking the tree apart
//! into `let` bindings and `if` statements. Instead, `cond.then(|| ...)` fits
//! inline. This is done through a private trait on the method rather than a
//! [`From`] impl, since an `Option` is not an item and should not convert into
//! one anywhere else.
//!
//! # Macros
//!
//! Each item has a macro that is usually shorter than the equivalent builder
//! chain. These macros are linked in the docs for the item type, and are
//! re-exported in the [`nebula`](crate) crate root for convenience.
//!
//! ```
//! use nebula::{button, frame, msg};
//!
//! let confirm = msg!(
//!     ephemeral: true,
//!
//!     frame!(
//!         color: 0x5865f2,
//!
//!         "**Delete the channel?**",
//!         "This cannot be undone.",
//!         [
//!             button!(danger: "delete", "Delete"),
//!             button!(secondary: "cancel", "Cancel"),
//!         ],
//!     ),
//! );
//! ```
//!
//! Named options must come before positional items, as the first positional
//! item ends the option list. Writing `msg!("hi", ephemeral: true)` is a
//! compile error; write `msg!(ephemeral: true, "hi")` instead.

mod button;
mod frame;
mod msg;
mod row;
mod text;

pub use button::Button;
pub use frame::{Frame, FrameItem};
pub use msg::{Msg, MsgItem};
pub use row::Row;
pub use text::Text;
