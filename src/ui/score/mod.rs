use bevy::prelude::*;

use crate::PausedState;

use self::{
    events::ScoreUpEvent,
    resources::PlayerScore,
    systems::{
        layout::{despawn_score_count, spawn_score_count},
        updates::{trigger_score_update, update_score, update_score_text},
    },
};

pub mod components;
mod events;
pub mod resources;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScoreUpEvent>()
            .insert_resource(PlayerScore(0))
            .add_systems(OnEnter(PausedState::Running), spawn_score_count)
            .add_systems(OnEnter(PausedState::GameOver), despawn_score_count)
            .add_systems(
                Update,
                (trigger_score_update, update_score, update_score_text)
                    .run_if(in_state(PausedState::Running)),
            );
    }
}
