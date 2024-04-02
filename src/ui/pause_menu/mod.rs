use bevy::prelude::*;

use self::components::PauseMenu;

mod components;

pub struct PauseMenuPlugin;

fn buld_pause_menu(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: BackgroundColor::from(Color::RED),
                ..default()
            },
            PauseMenu,
        ))
        .id()
}

fn spawn_pause_menu(mut commands: Commands) {
    buld_pause_menu(&mut commands);
}

fn despawn_pause_menu(mut commands: Commands, q_pause_menu: Query<Entity, With<PauseMenu>>) {
    if let Ok(menu_ent) = q_pause_menu.get_single() {
        commands.entity(menu_ent).despawn_recursive();
    }
}

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::PausedState::Paused), spawn_pause_menu)
            .add_systems(OnExit(crate::PausedState::Paused), despawn_pause_menu);
    }
}
