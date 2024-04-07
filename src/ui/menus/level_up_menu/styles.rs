use bevy::prelude::*;

pub struct UpgradeMenuStyle(pub Style);

impl Default for UpgradeMenuStyle {
    fn default() -> Self {
        UpgradeMenuStyle(Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(8.),
            ..Style::default()
        })
    }
}

impl UpgradeMenuStyle {
    pub fn bg_color() -> Color {
        // Color::RED
        Color::NONE
    }
}
