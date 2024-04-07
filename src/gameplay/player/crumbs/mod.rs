use bevy::prelude::*;

use crate::gameplay::states::PausedState;

use self::systems::{collect_crumbs, drop_crumbs};

pub mod bundles;
pub mod components;
pub mod systems;

pub struct ExpCrumbPlugin;

impl Plugin for ExpCrumbPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (collect_crumbs, drop_crumbs).run_if(in_state(PausedState::Running)),
        );
    }
}
