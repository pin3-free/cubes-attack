use bevy::prelude::*;

use crate::ui::{
    menus::{
        components::{QuitButton, ResetButton, StyledButton},
        game_over_menu::components::GameOverMenu,
        styles::{ButtonStyle, MenuStyle},
    },
    score::resources::PlayerScore,
};

fn build_game_over_menu(commands: &mut Commands, score: Res<PlayerScore>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: MenuStyle::default().0,
                background_color: MenuStyle::bg_color().into(),
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
            parent.spawn(TextBundle::from_section(
                format!("Your score: {score}", score = score.0),
                TextStyle {
                    font_size: 50.,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ));
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

pub fn spawn_game_over_menu(mut commands: Commands, score: Res<PlayerScore>) {
    build_game_over_menu(&mut commands, score);
}
