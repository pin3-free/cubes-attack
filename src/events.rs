use bevy::prelude::{Event, Vec2};

use crate::gameplay::components::{Damage, Shooter};

#[derive(Event)]
pub struct ShootEvent {
    pub source: Vec2,
    pub target: Vec2,
    pub damage: Damage,
    pub shooter: Shooter,
}
