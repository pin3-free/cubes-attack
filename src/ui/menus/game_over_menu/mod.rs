use bevy::prelude::*;

use crate::PausedState;

use self::{components::GameOverMenu, systems::layout::spawn_game_over_menu};

use super::despawn_menu;

pub mod components;
pub mod systems;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PausedState::GameOver), spawn_game_over_menu)
            .add_systems(OnExit(PausedState::GameOver), despawn_menu::<GameOverMenu>);
    }
}
