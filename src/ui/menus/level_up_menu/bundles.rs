use bevy::prelude::*;

use crate::ui::menus::{components::StyledButton, styles::ButtonStyle};

use super::{
    components::{UpgradeButton, UpgradeVariantComponent},
    systems::layout::UpgradeVariant,
};

#[derive(Bundle)]
pub struct UpgradeButtonBundle {
    pub button: ButtonBundle,
    pub upgrade_var: UpgradeVariantComponent,
    pub marker: UpgradeButton,
    pub styled: StyledButton,
}

impl Default for UpgradeButtonBundle {
    fn default() -> Self {
        UpgradeButtonBundle::with_variant(UpgradeVariant::Damage)
    }
}

impl UpgradeButtonBundle {
    pub fn with_variant(variant: UpgradeVariant) -> Self {
        Self {
            button: ButtonBundle {
                style: ButtonStyle::default().0,
                background_color: ButtonStyle::bg_color().into(),
                ..default()
            },
            upgrade_var: UpgradeVariantComponent(variant),
            marker: UpgradeButton,
            styled: StyledButton,
        }
    }

    pub fn get_text(variant: &UpgradeVariant) -> TextBundle {
        match variant {
            UpgradeVariant::Speed => ButtonStyle::text("Speed"),
            UpgradeVariant::Damage => ButtonStyle::text("Damage"),
            UpgradeVariant::Health => ButtonStyle::text("Health"),
            UpgradeVariant::FireRate => ButtonStyle::text("Fire rate"),
            UpgradeVariant::ShotSpeed => ButtonStyle::text("Shot speed"),
        }
    }
}
