use bevy::prelude::*;

use crate::PausedState;

use self::{
    events::PlayerMoveEvent,
    systems::{draw_player, move_player},
};

pub mod bundles;
pub mod components;
pub mod events;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMoveEvent>()
            .add_systems(Startup, draw_player)
            .add_systems(Update, (move_player).run_if(in_state(PausedState::Running)));
    }
}
