use core::time;

use bevy::{prelude::*, time::Stopwatch};

use crate::gameplay::{
    bundles::ShooterBundle,
    components::{
        Damage, Health, PointWorth, ReloadStopwatch, ReloadTime, RemoveOnReset, Shooter, ShotSpeed,
        Speed,
    },
};

use super::components::Enemy;

#[derive(Bundle, Clone)]
pub struct EnemyBundle {
    speed: Speed,
    marker: Enemy,
    shooter_marker: Shooter,
    hp: Health,
    sprite: SpriteBundle,
    remove_on_reset: RemoveOnReset,
    point_worth: PointWorth,
}

impl EnemyBundle {
    pub fn with_transform(self, transform: Transform) -> Self {
        Self {
            sprite: SpriteBundle {
                transform,
                ..self.sprite
            },
            ..self
        }
    }
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            speed: Speed(100.),
            hp: Health(10),
            point_worth: PointWorth(5),
            marker: Enemy,
            shooter_marker: Shooter::Enemy,
            remove_on_reset: RemoveOnReset,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(50., 50.)),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct ShooterEnemyBundle {
    enemy: EnemyBundle,
    shooter: ShooterBundle,
}

impl ShooterEnemyBundle {
    pub fn with_transform(self, transform: Transform) -> Self {
        Self {
            enemy: self.enemy.with_transform(transform),
            shooter: self.shooter,
        }
    }
}

impl Default for ShooterEnemyBundle {
    fn default() -> Self {
        Self {
            enemy: EnemyBundle {
                hp: Health(20),
                point_worth: PointWorth(10),
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: Color::PURPLE,
                        custom_size: Some(Vec2::new(50., 50.)),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            shooter: ShooterBundle {
                damage: Damage(5),
                since_last_reload: ReloadStopwatch(Stopwatch::new()),
                reload_time: ReloadTime(time::Duration::from_secs(2)),
                shot_speed: ShotSpeed(300.),
            },
        }
    }
}
