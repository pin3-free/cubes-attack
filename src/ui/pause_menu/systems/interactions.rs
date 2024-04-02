use bevy::prelude::*;

use crate::ui::pause_menu::{components::StyledButton, styles::ButtonStyle};

pub fn interact_styled_button<T: Component>(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<T>)>,
) {
    if let Ok((interaction, mut bg_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = ButtonStyle::press_bg_color().into();
            }
            Interaction::Hovered => {
                *bg_color = ButtonStyle::hover_bg_color().into();
            }
            Interaction::None => {
                *bg_color = ButtonStyle::bg_color().into();
            }
        }
    }
}
