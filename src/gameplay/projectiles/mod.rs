use bevy::prelude::*;

use crate::gameplay::states::PausedState;

use self::systems::{bullet_collision_processing, bullet_spawner, move_bullets};

pub mod bundles;
pub mod components;
pub mod events;
pub mod systems;

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (bullet_spawner, move_bullets, bullet_collision_processing)
                .run_if(in_state(PausedState::Running)),
        );
    }
}
