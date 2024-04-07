use bevy::prelude::*;

use super::systems::layout::UpgradeVariant;

#[derive(Component)]
pub struct UpgradeMenu;

#[derive(Component)]
pub struct UpgradeVariantComponent(pub UpgradeVariant);

#[derive(Component)]
pub struct UpgradeButton;
