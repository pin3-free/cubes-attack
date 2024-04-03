use bevy::prelude::*;

use crate::ui::menus::{
    components::{QuitButton, ResetButton, ResumeButton, StyledButton},
    pause_menu::components::PauseMenu,
    styles::{ButtonStyle, MenuStyle},
};

fn buld_pause_menu(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: MenuStyle::default().0,
                background_color: MenuStyle::bg_color().into(),
                ..default()
            },
            PauseMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: ButtonStyle::default().0,
                        background_color: ButtonStyle::bg_color().into(),
                        ..default()
                    },
                    ResumeButton,
                    StyledButton,
                ))
                .with_children(|parent| {
                    parent.spawn(ButtonStyle::text("Resume"));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: ButtonStyle::default().0,
                        background_color: ButtonStyle::bg_color().into(),
                        ..default()
                    },
                    ResetButton,
                    StyledButton,
                ))
                .with_children(|parent| {
                    parent.spawn(ButtonStyle::text("Reset"));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: ButtonStyle::default().0,
                        background_color: ButtonStyle::bg_color().into(),
                        ..default()
                    },
                    QuitButton,
                    StyledButton,
                ))
                .with_children(|parent| {
                    parent.spawn(ButtonStyle::text("Quit"));
                });
        })
        .id()
}

pub fn spawn_pause_menu(mut commands: Commands) {
    buld_pause_menu(&mut commands);
}
