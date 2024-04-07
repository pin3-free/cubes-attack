use bevy::prelude::*;

use crate::gameplay::states::PausedState;

use self::{
    crumbs::ExpCrumbPlugin,
    events::PlayerMoveEvent,
    resources::PlayerExperience,
    systems::{draw_player, move_player},
};

pub mod bundles;
pub mod components;
pub mod crumbs;
pub mod events;
pub mod resources;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExpCrumbPlugin)
            .insert_resource(PlayerExperience(0))
            .add_event::<PlayerMoveEvent>()
            .add_systems(Startup, draw_player)
            .add_systems(Update, (move_player).run_if(in_state(PausedState::Running)));
    }
}
