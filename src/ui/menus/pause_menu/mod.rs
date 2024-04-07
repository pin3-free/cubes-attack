use bevy::prelude::*;

use crate::gameplay::states::GameState;

use self::components::PauseMenu;
use self::systems::layout::spawn_pause_menu;

use super::despawn_menu;

mod components;
mod styles;
mod systems;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), spawn_pause_menu)
            .add_systems(OnExit(GameState::Paused), despawn_menu::<PauseMenu>);
    }
}
