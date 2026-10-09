//! Contains action types, which carry what a user did and let the bot respond.
//!
//! Nebula calls Discord's
//! [interactions](https://docs.discord.com/developers/interactions/overview)
//! *actions*. Only actions performed in guilds are supported for now
//! ([#1](https://github.com/jackhexed/nebula/issues/1)).

use std::sync::Arc;

use crate::{
    Snowflake,
    action::common::Common,
    backend::ResponseSink,
    cache::Cache,
    model::{Guild, ModelCtx, User},
    rest::Rest,
};

mod common;
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
        let cache = Cache::new();

        let action_id = Snowflake::from(interaction.id);
        let action_token = interaction.token.into_boxed_str();

        // for now we only support guild interactions
        let model_ctx = ModelCtx::new(Arc::clone(&rest), Arc::clone(&cache));
        let guild_id = Snowflake::from(interaction.guild_id.unwrap());
        let guild = Guild::new(model_ctx, guild_id);

        let model_ctx = ModelCtx::new(Arc::clone(&rest), Arc::clone(&cache));
        let user_id = Snowflake::from(interaction.member.unwrap().user.unwrap().id);
        let user = User::new(model_ctx, user_id);

        let common = Common::new(rest, cache, sink, action_id, action_token, guild, user);

        match interaction.kind {
            tw::InteractionType::ApplicationCommand => {
                let Some(tw::InteractionData::ApplicationCommand(data)) = interaction.data else {
                    unreachable!()
                };

                Action::Slash(SlashCtx::parse(common, data))
            }

            _ => unreachable!(),
        }
    }
}
