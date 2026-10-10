use crate::{
    action::ActionCtx,
    error::Error,
    item::Msg,
    model::{Guild, User},
};

mod tw {
    pub use twilight_model::application::interaction::message_component::MessageComponentInteractionData;
    pub use twilight_model::channel::message::component::ComponentType;
}

/// A button action, which occurs when a user clicks a button in a message.
pub struct ButtonCtx<S = ()>
where
    S: Send + Sync + 'static,
{
    ctx: ActionCtx<S>,

    custom_id: Box<str>,
}

impl<S> ButtonCtx<S>
where
    S: Send + Sync + 'static,
{
    /// Returns the bot's state.
    pub fn state(&self) -> &S {
        self.ctx.state()
    }

    /// Replies to the button action with the given message.
    pub async fn reply(&self, msg: impl Into<Msg>) -> Result<(), Error> {
        self.ctx.reply(msg).await
    }

    /// Returns the guild the command was run in.
    pub fn guild(&self) -> &Guild {
        self.ctx.guild()
    }

    /// Returns the user who ran the command.
    pub fn user(&self) -> &User {
        self.ctx.user()
    }

    /// Returns the custom ID of the button that was clicked.
    pub fn custom_id(&self) -> &str {
        &self.custom_id
    }

    /// Parses a button action from the given action context and message
    /// component data.
    pub(crate) fn parse(ctx: ActionCtx<S>, data: Box<tw::MessageComponentInteractionData>) -> Self {
        assert_eq!(data.component_type, tw::ComponentType::Button);

        let custom_id = data.custom_id.into_boxed_str();

        Self { ctx, custom_id }
    }
}
