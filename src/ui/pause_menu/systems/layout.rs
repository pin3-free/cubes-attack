use bevy::prelude::*;

use crate::ui::pause_menu::{
    components::{GameOverMenu, PauseMenu, QuitButton, ResetButton, ResumeButton, StyledButton},
    styles::{ButtonStyle, MainMenuStyle},
};

fn buld_pause_menu(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: MainMenuStyle::default().0,
                background_color: MainMenuStyle::bg_color().into(),
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

fn build_game_over_menu(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: MainMenuStyle::default().0,
                background_color: MainMenuStyle::bg_color().into(),
                ..default()
            },
            GameOverMenu,
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle::from_section(
                "Game over",
                TextStyle {
                    font_size: 100.,
                    color: Color::RED,
                    ..Default::default()
                },
            ),));
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

pub fn spawn_game_over_menu(mut commands: Commands) {
    build_game_over_menu(&mut commands);
}

pub fn despawn_menu<T: Component>(mut commands: Commands, q_pause_menu: Query<Entity, With<T>>) {
    if let Ok(menu_ent) = q_pause_menu.get_single() {
        commands.entity(menu_ent).despawn_recursive();
    }
}
