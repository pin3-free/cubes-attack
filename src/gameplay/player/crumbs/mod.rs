use bevy::prelude::*;

use crate::gameplay::states::GameState;

use self::{
    events::LevelUpEvent,
    systems::{collect_crumbs, drop_crumbs, level_up},
};

pub mod bundles;
pub mod components;
pub mod events;
pub mod systems;

pub struct ExpCrumbPlugin;

impl Plugin for ExpCrumbPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelUpEvent>().add_systems(
            Update,
            (collect_crumbs, drop_crumbs, level_up).run_if(in_state(GameState::Running)),
        );
    }
}
