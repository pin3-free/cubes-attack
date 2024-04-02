use bevy::prelude::*;

use self::components::{QuitButton, ResumeButton};
use self::systems::interactions::interact_styled_button;
use self::systems::layout::{despawn_pause_menu, spawn_pause_menu};

mod components;
mod styles;
mod systems;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                interact_styled_button::<ResumeButton>,
                interact_styled_button::<QuitButton>,
            )
                .run_if(in_state(crate::PausedState::Paused)),
        )
        .add_systems(OnEnter(crate::PausedState::Paused), spawn_pause_menu)
        .add_systems(OnExit(crate::PausedState::Paused), despawn_pause_menu);
    }
}
