use bevy::prelude::*;

pub struct MainMenuStyle(pub Style);
pub struct ButtonStyle(pub Style);

impl Default for MainMenuStyle {
    fn default() -> Self {
        MainMenuStyle(Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            padding: UiRect::all(Val::Px(200.)),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.),
            ..Style::default()
        })
    }
}
impl Default for ButtonStyle {
    fn default() -> Self {
        ButtonStyle(Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Px(200.),
            height: Val::Px(80.),
            ..default()
        })
    }
}

impl MainMenuStyle {
    pub fn bg_color() -> Color {
        Color::RED
    }
}

impl ButtonStyle {
    pub fn text_style() -> TextStyle {
        TextStyle {
            font_size: 32.,
            color: Color::WHITE,
            ..default()
        }
    }

    pub fn bg_color() -> Color {
        Color::rgb(0.15, 0.15, 0.15)
    }

    pub fn hover_bg_color() -> Color {
        Color::rgb(0.25, 0.25, 0.25)
    }

    pub fn press_bg_color() -> Color {
        Color::rgb(0.35, 0.75, 0.35)
    }

    pub fn text(text: &str) -> TextBundle {
        TextBundle::from_section(text, Self::text_style())
    }
}
