use bevy::prelude::{Event, Vec2};

use crate::gameplay::components::{Damage, Shooter, ShotSpeed};

#[derive(Event)]
pub struct ShootEvent {
    pub source: Vec2,
    pub target: Vec2,
    pub damage: Damage,
    pub shooter: Shooter,
    pub bullet_speed: ShotSpeed,
}
