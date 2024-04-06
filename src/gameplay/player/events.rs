use bevy::prelude::*;

use crate::gameplay::MoveDirection;

#[derive(Event)]
pub struct PlayerMoveEvent(pub MoveDirection);
