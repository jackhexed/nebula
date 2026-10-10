//! Contains action types, which carry what a user did and let the bot respond.
//!
//! Nebula calls Discord's
//! [interactions](https://docs.discord.com/developers/interactions/overview)
//! *actions*. Only actions performed in guilds are supported for now
//! ([#1](https://github.com/jackhexed/nebula/issues/1)).

use std::sync::Arc;

use crate::{
    Snowflake,
    backend::{Response, ResponseKind, ResponseSink},
    ctx::Ctx,
    error::Error,
    item::Msg,
    model::{Guild, User},
    rest::Rest,
    snowflake::ActionMarker,
};

mod slash;

pub use slash::{SlashArg, SlashCtx};

mod tw {
    pub use twilight_model::application::interaction::Interaction;
    pub use twilight_model::application::interaction::InteractionData;
    pub use twilight_model::application::interaction::InteractionType;
}

/// An action a user performed, such as running a slash command.
///
/// Each variant holds the context for its kind of action, such as
/// [`SlashCtx`]. Every context exposes the information all actions share —
/// the action's snowflake and token, the guild, and the user — alongside
/// whatever is specific to its kind.
///
/// [Discord docs](https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-object-interaction-structure)
pub enum Action {
    /// A slash command was run.
    Slash(SlashCtx),
}

impl Action {
    /// Parses a Twilight interaction into an action.
    ///
    /// # Panics
    ///
    /// This function will panic if the interaction was not performed in a
    /// guild, or is not an application command.
    pub(crate) fn parse(rest: Arc<Rest>, sink: ResponseSink, interaction: tw::Interaction) -> Self {
        let ctx = Ctx::new(rest);

        let action_id = Snowflake::from(interaction.id);
        let action_token = interaction.token.into_boxed_str();

        // for now we only support guild interactions
        let guild_id = Snowflake::from(interaction.guild_id.unwrap());
        let guild = Guild::new(&ctx, guild_id);

        let user_id = Snowflake::from(interaction.member.unwrap().user.unwrap().id);
        let user = User::new(&ctx, user_id);

        let ctx = ActionCtx::new(ctx, sink, action_id, action_token, guild, user);

        match interaction.kind {
            tw::InteractionType::ApplicationCommand => {
                let Some(tw::InteractionData::ApplicationCommand(data)) = interaction.data else {
                    unreachable!()
                };

                Action::Slash(SlashCtx::parse(ctx, data))
            }

            _ => unreachable!(),
        }
    }
}

/// A context for an action, which holds the action's ID and token, the guild
/// it was performed in, and the user who performed it.
pub(crate) struct ActionCtx {
    ctx: Ctx,
    sink: ResponseSink,

    action_id: Snowflake<ActionMarker>,
    action_token: Box<str>,

    guild: Guild,
    user: User,
}

impl ActionCtx {
    /// Creates a new ActionCtx.
    pub(crate) fn new(
        ctx: Ctx,
        sink: ResponseSink,
        action_id: Snowflake<ActionMarker>,
        action_token: Box<str>,
        guild: Guild,
        user: User,
    ) -> Self {
        Self {
            ctx,
            sink,
            action_id,
            action_token,
            guild,
            user,
        }
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
}

impl From<&ActionCtx> for Ctx {
    fn from(ctx: &ActionCtx) -> Self {
        ctx.ctx.clone()
    }
}
