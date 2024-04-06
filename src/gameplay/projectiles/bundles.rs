use bevy::prelude::*;

use crate::{Damage, MyDirection, RemoveOnReset, Shooter, Speed};

use super::components::{Bullet, BulletLifetimeTimer};

#[derive(Bundle)]
pub struct BulletBundle {
    pub speed: Speed,
    pub marker: Bullet,
    pub direction: MyDirection,
    pub damage: Damage,
    pub lifetime: BulletLifetimeTimer,
    pub sprite: SpriteBundle,
    pub shooter: Shooter,
    pub remove_on_reset: RemoveOnReset,
}

impl Default for BulletBundle {
    fn default() -> Self {
        Self {
            speed: Speed(300.),
            marker: Bullet,
            direction: MyDirection(Vec2::new(1., 0.)),
            lifetime: BulletLifetimeTimer(Timer::from_seconds(20., TimerMode::Once)),
            shooter: Shooter::Player,
            damage: Damage(5),
            remove_on_reset: RemoveOnReset,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(10., 10.)),
                    ..default()
                },
                ..default()
            },
        }
    }
}
