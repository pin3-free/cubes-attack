use bevy::prelude::*;

use super::systems::layout::UpgradeVariant;

#[derive(Event)]
pub struct UpgradeStatEvent {
    pub variant: UpgradeVariant,
}
