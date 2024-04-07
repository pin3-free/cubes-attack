use bevy::prelude::*;

use crate::gameplay::states::GameState;

use self::{
    game_over_menu::GameOverPlugin,
    pause_menu::PauseMenuPlugin,
    systems::interactions::{
        interact_styled_button, interact_with_quit_button, interact_with_reset_button,
        interact_with_resume_button,
    },
};

pub mod components;
pub mod game_over_menu;
pub mod level_up_menu;
pub mod pause_menu;
pub mod styles;
mod systems;

pub fn despawn_menu<T: Component>(mut commands: Commands, q_pause_menu: Query<Entity, With<T>>) {
    if let Ok(menu_ent) = q_pause_menu.get_single() {
        commands.entity(menu_ent).despawn_recursive();
    }
}

pub struct GlobalMenuPlugin;

impl Plugin for GlobalMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameOverPlugin, PauseMenuPlugin))
            .add_systems(
                Update,
                (
                    interact_styled_button,
                    interact_with_quit_button,
                    interact_with_reset_button,
                    interact_with_resume_button,
                )
                    .run_if(not(in_state(GameState::Running))),
            );
    }
}
