use bevy::prelude::*;

use crate::gameplay::states::GameState;

use self::{
    components::UpgradeMenu,
    events::UpgradeStatEvent,
    systems::{interact_upgrade_button, layout::spawn_upgrade_menu, process_upgrade_event},
};

use super::despawn_menu;

pub mod bundles;
pub mod components;
pub mod events;
pub mod styles;
pub mod systems;

pub struct UpgradeMenuPlugin;

impl Plugin for UpgradeMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpgradeStatEvent>()
            .add_systems(OnEnter(GameState::Upgrading), spawn_upgrade_menu)
            .add_systems(OnExit(GameState::Upgrading), despawn_menu::<UpgradeMenu>)
            .add_systems(
                Update,
                (interact_upgrade_button, process_upgrade_event)
                    .run_if(in_state(GameState::Upgrading)),
            );
    }
}
