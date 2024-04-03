use bevy::prelude::*;

use crate::PausedState;

use self::components::PauseMenu;
use self::systems::layout::spawn_pause_menu;

use super::despawn_menu;

mod components;
mod styles;
mod systems;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PausedState::Paused), spawn_pause_menu)
            .add_systems(OnExit(PausedState::Paused), despawn_menu::<PauseMenu>);
    }
}
