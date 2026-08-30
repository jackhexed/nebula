use std::sync::Arc;

use crate::{
    Guild, Snowflake, User, cache::Cache, model::ModelCtx, shared::Shared, snowflake::ActionMarker,
};

mod slash;

pub use slash::SlashCtx;

mod tw {
    pub use twilight_model::application::interaction::Interaction;
    pub use twilight_model::application::interaction::InteractionData;
    pub use twilight_model::application::interaction::InteractionType;
}

/// An action that a user has performed, such as a slash command.
///
/// This is a wrapper around the underlying action context, which contains
/// information about the action.
pub enum Action {
    Slash(SlashCtx),
}

/// A context for actions, providing access to shared state and cache, along
/// with storing and parsing common action information.
pub struct ActionCtx {
    shared: Arc<Shared>,
    cache: Arc<Cache>,

    action_id: Snowflake<ActionMarker>,
    action_token: Box<str>,

    guild: Guild,
    user: User,
}

impl ActionCtx {
    /// Returns a reference to the shared state.
    pub fn shared(&self) -> &Arc<Shared> {
        &self.shared
    }

    /// Returns a reference to the cache.
    pub fn cache(&self) -> &Arc<Cache> {
        &self.cache
    }

    /// Returns the snowflake of the action.
    pub fn action_id(&self) -> Snowflake<ActionMarker> {
        self.action_id
    }

    /// Returns the token of the action.
    pub fn action_token(&self) -> &str {
        &self.action_token
    }

    /// Returns a reference to the guild that the action was performed in.
    pub fn guild(&self) -> &Guild {
        &self.guild
    }

    /// Returns a reference to the user that performed the action.
    pub fn user(&self) -> &User {
        &self.user
    }

    /// Returns a model context for the action.
    pub fn model_ctx(&self) -> ModelCtx {
        ModelCtx::new(Arc::clone(&self.shared), Arc::clone(&self.cache))
    }

    /// Parses an action from a twilight interaction.
    pub fn parse(shared: Arc<Shared>, interaction: tw::Interaction) -> Action {
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
