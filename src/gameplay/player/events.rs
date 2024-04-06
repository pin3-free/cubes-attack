use bevy::prelude::*;

#[derive(Event)]
pub struct PlayerMoveEvent(pub crate::MoveDirection);
