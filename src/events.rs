use bevy::prelude::*;

use crate::components::*;

#[derive(Event)]
pub struct ShootEvent {
    pub source: Vec2,
    pub target: Vec2,
    pub damage: Damage,
    pub shooter: Shooter,
}
