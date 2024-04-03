use bevy::prelude::*;

use crate::PausedState;

use self::components::{GameOverMenu, PauseMenu, QuitButton, ResetButton, ResumeButton};
use self::systems::interactions::{
    interact_styled_button, interact_with_quit_button, interact_with_reset_button,
    interact_with_resume_button,
};
use self::systems::layout::{despawn_menu, spawn_game_over_menu, spawn_pause_menu};

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
                interact_styled_button::<ResetButton>,
                interact_with_quit_button,
                interact_with_resume_button,
                interact_with_reset_button,
            )
                .run_if(not(in_state(PausedState::Running))),
        )
        .add_systems(OnEnter(PausedState::Paused), spawn_pause_menu)
        .add_systems(OnExit(PausedState::Paused), despawn_menu::<PauseMenu>)
        .add_systems(OnEnter(PausedState::GameOver), spawn_game_over_menu)
        .add_systems(OnExit(PausedState::GameOver), despawn_menu::<GameOverMenu>);
    }
}
