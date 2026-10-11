//! Contains action types, which carry what a user did and let the bot respond.
//!
//! Nebula calls Discord's
//! [interactions](https://docs.discord.com/developers/interactions/overview)
//! *actions*. Only actions performed in guilds are supported for now
//! ([#1](https://github.com/jackhexed/nebula/issues/1)).

use std::sync::Arc;

use crate::{
    Snowflake,
    action::button::ButtonCtx,
    backend::{Response, ResponseKind, ResponseSink},
    ctx::Ctx,
    error::Error,
    item::Msg,
    model::{Guild, User},
    rest::Rest,
    snowflake::ActionMarker,
};

mod button;
mod slash;

pub use slash::{SlashArg, SlashCtx};

mod tw {
    pub use twilight_model::application::interaction::Interaction;
    pub use twilight_model::application::interaction::InteractionData;
    pub use twilight_model::application::interaction::InteractionType;
    pub use twilight_model::channel::message::component::ComponentType;
}

/// An action a user performed, such as running a slash command.
///
/// Each variant holds the context for its kind of action, such as
/// [`SlashCtx`]. Every context exposes the information all actions share —
/// the action's snowflake and token, the guild, and the user — alongside
/// whatever is specific to its kind.
///
/// [Discord docs](https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-object-interaction-structure)
pub enum Action<S = ()>
where
    S: Send + Sync + 'static,
{
    /// A slash command was run.
    Slash(SlashCtx<S>),

    /// A button was clicked.
    Button(ButtonCtx<S>),
}

impl<S> Action<S>
where
    S: Send + Sync + 'static,
{
    /// Parses a Twilight interaction into an action.
    ///
    /// Will return none if the interaction is not a supported variant.
    pub(crate) fn parse(
        rest: Arc<Rest>,
        sink: ResponseSink,
        state: Arc<S>,
        interaction: tw::Interaction,
    ) -> Option<Self>
    where
        S: Send + Sync + 'static,
    {
        let ctx = Ctx::new(rest);

        let action_id = Snowflake::from(interaction.id);
        let action_token = interaction.token.into_boxed_str();

        // for now we only support guild interactions
        let guild_id = Snowflake::from(interaction.guild_id?);
        let guild = Guild::new(&ctx, guild_id);

        let user_id = Snowflake::from(interaction.member?.user?.id);
        let user = User::new(&ctx, user_id);

        let ctx = ActionCtx::new(ctx, sink, state, action_id, action_token, guild, user);

        match interaction.kind {
            tw::InteractionType::ApplicationCommand => {
                let Some(tw::InteractionData::ApplicationCommand(data)) = interaction.data else {
                    unreachable!()
                };

                Some(Action::Slash(SlashCtx::parse(ctx, data)))
            }

            tw::InteractionType::MessageComponent => {
                let Some(tw::InteractionData::MessageComponent(data)) = interaction.data else {
                    unreachable!()
                };

                match data.component_type {
                    tw::ComponentType::Button => Some(Action::Button(ButtonCtx::parse(ctx, data))),

                    _ => None,
                }
            }

            _ => None,
        }
    }
}

/// A context for an action, which holds the action's ID and token, the guild
/// it was performed in, and the user who performed it.
pub(crate) struct ActionCtx<S>
where
    S: Send + Sync + 'static,
{
    ctx: Ctx,
    sink: ResponseSink,
    state: Arc<S>,

    action_id: Snowflake<ActionMarker>,
    action_token: Box<str>,

    guild: Guild,
    user: User,
}

impl<S> ActionCtx<S>
where
    S: Send + Sync + 'static,
{
    /// Creates a new ActionCtx.
    pub(crate) fn new(
        ctx: Ctx,
        sink: ResponseSink,
        state: Arc<S>,
        action_id: Snowflake<ActionMarker>,
        action_token: Box<str>,
        guild: Guild,
        user: User,
    ) -> Self {
        Self {
            ctx,
            sink,
            state,
            action_id,
            action_token,
            guild,
            user,
        }
    }

    /// Returns the bot's state.
    pub(crate) fn state(&self) -> &S {
        &self.state
    }

    /// Returns the guild the action was performed in.
    pub(crate) fn guild(&self) -> &Guild {
        &self.guild
    }

    /// Returns the user who performed the action.
    pub(crate) fn user(&self) -> &User {
        &self.user
    }

    /// Replies to the action with the given message.
    pub(crate) async fn reply(&self, msg: impl Into<Msg>) -> Result<(), Error> {
        let msg = msg.into();
        let response = Response::new(self.action_id, &self.action_token, ResponseKind::Reply(msg));

        self.sink.send(self.ctx.rest(), response).await
    }

    /// Defers a reply to the action.
    pub(crate) async fn defer_reply(&self) -> Result<(), Error> {
        let response = Response::new(self.action_id, &self.action_token, ResponseKind::DeferReply);

        self.sink.send(self.ctx.rest(), response).await
    }

    /// Edit the original response with the given message.
    pub(crate) async fn edit(&self, msg: impl Into<Msg>) -> Result<(), Error> {
        let msg = msg.into();
        let response = Response::new(self.action_id, &self.action_token, ResponseKind::Edit(msg));

        self.sink.send(self.ctx.rest(), response).await
    }

    /// Defers an edit to the original response.
    pub(crate) async fn defer_edit(&self) -> Result<(), Error> {
        let response = Response::new(self.action_id, &self.action_token, ResponseKind::DeferEdit);

        self.sink.send(self.ctx.rest(), response).await
    }
}

impl<S> From<&ActionCtx<S>> for Ctx
where
    S: Send + Sync + 'static,
{
    fn from(ctx: &ActionCtx<S>) -> Self {
        ctx.ctx.clone()
    }
}
