use bevy::prelude::*;

use crate::ui::pause_menu::{
    components::{PauseMenu, QuitButton, ResumeButton, StyledButton},
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
    let menu = buld_pause_menu(&mut commands);
    dbg!(menu);
}

pub fn despawn_pause_menu(mut commands: Commands, q_pause_menu: Query<Entity, With<PauseMenu>>) {
    if let Ok(menu_ent) = q_pause_menu.get_single() {
        commands.entity(menu_ent).despawn_recursive();
    }
}
