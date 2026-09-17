//! Contains action types, which carry what a user did and let the bot respond.
//!
//! Nebula calls Discord's
//! [interactions](https://docs.discord.com/developers/interactions/overview)
//! *actions*. Only actions performed in guilds are supported for now
//! ([#1](https://github.com/jackhexed/nebula/issues/1)).

use std::sync::Arc;

use crate::{
    Snowflake,
    backend::ResponseSink,
    cache::Cache,
    model::ModelCtx,
    model::{Guild, User},
    shared::Shared,
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
/// Each variant holds the context for its kind of action, and every context
/// derefs to [`ActionCtx`] for the information all actions share.
///
/// [Discord docs](https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-object-interaction-structure)
pub enum Action {
    /// A slash command was run.
    Slash(SlashCtx),
}

/// The information every action shares.
///
/// This is never received on its own. Each [`Action`] variant's context, such
/// as [`SlashCtx`], derefs to it.
pub struct ActionCtx {
    shared: Arc<Shared>,
    cache: Arc<Cache>,
    sink: ResponseSink,

    action_id: Snowflake<ActionMarker>,
    action_token: Box<str>,

    guild: Guild,
    user: User,
}

impl ActionCtx {
    /// Returns the shared state.
    pub(crate) fn shared(&self) -> &Arc<Shared> {
        &self.shared
    }

    /// Returns the action's cache.
    pub(crate) fn cache(&self) -> &Arc<Cache> {
        &self.cache
    }

    /// Returns the action's response sink.
    pub(crate) fn sink(&self) -> &ResponseSink {
        &self.sink
    }

    /// Returns the action's snowflake.
    pub fn action_id(&self) -> Snowflake<ActionMarker> {
        self.action_id
    }

    /// Returns the action's token.
    ///
    /// The token is used to respond to the action, and is valid for 15
    /// minutes.
    pub fn action_token(&self) -> &str {
        &self.action_token
    }

    /// Returns the guild the action was performed in.
    pub fn guild(&self) -> &Guild {
        &self.guild
    }

    /// Returns the user who performed the action.
    pub fn user(&self) -> &User {
        &self.user
    }

    /// Returns a model context for the action.
    pub(crate) fn model_ctx(&self) -> ModelCtx {
        ModelCtx::new(Arc::clone(&self.shared), Arc::clone(&self.cache))
    }

    /// Parses a Twilight interaction into an action.
    ///
    /// # Panics
    ///
    /// This function will panic if the interaction was not performed in a
    /// guild, or is not an application command.
    pub(crate) fn parse(
        shared: Arc<Shared>,
        sink: ResponseSink,
        interaction: tw::Interaction,
    ) -> Action {
        let cache = Cache::new();

        let action_id = Snowflake::from(interaction.id);
        let action_token = interaction.token.into_boxed_str();

        // for now we only support guild interactions
        let model_ctx = ModelCtx::new(Arc::clone(&shared), Arc::clone(&cache));
        let guild_id = Snowflake::from(interaction.guild_id.unwrap());
        let guild = Guild::new(model_ctx, guild_id);

        let model_ctx = ModelCtx::new(Arc::clone(&shared), Arc::clone(&cache));
        let user_id = Snowflake::from(interaction.member.unwrap().user.unwrap().id);
        let user = User::new(model_ctx, user_id);

        let action_ctx = Self {
            shared,
            cache,
            sink,

            action_id,
            action_token,

            guild,
            user,
        };

        match interaction.kind {
            tw::InteractionType::ApplicationCommand => {
                let Some(tw::InteractionData::ApplicationCommand(data)) = interaction.data else {
                    unreachable!()
                };

                Action::Slash(SlashCtx::parse(action_ctx, data))
            }

            _ => unreachable!(),
        }
    }
}
