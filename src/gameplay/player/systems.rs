use bevy::prelude::*;

use super::bundles::PlayerBundle;
use crate::gameplay::{components::Speed, MoveDirection};

use super::{components::Player, events::PlayerMoveEvent};

pub fn draw_player(mut commands: Commands) {
    commands.spawn(PlayerBundle::default());
}

pub fn move_player(
    mut q_player: Query<(&mut Transform, &Speed), With<Player>>,
    mut ev_move: EventReader<PlayerMoveEvent>,
    time: Res<Time>,
) {
    if let Ok((mut transform, speed)) = q_player.get_single_mut() {
        let delta = time.delta_seconds() * speed.0;
        ev_move
            .read()
            .for_each(|PlayerMoveEvent(direction)| match direction {
                MoveDirection::Up => {
                    transform.translation.y += delta;
                }
                MoveDirection::Down => {
                    transform.translation.y -= delta;
                }
                MoveDirection::Left => {
                    transform.translation.x -= delta;
                }
                MoveDirection::Right => {
                    transform.translation.x += delta;
                }
            });
    }
}
