use core::time;

use bevy::{prelude::*, time::Stopwatch};

use crate::gameplay::{
    bundles::ShooterBundle,
    components::{Damage, Health, ReloadStopwatch, ReloadTime, RemoveOnReset, Shooter, Speed},
};

use super::{components::Player, crumbs::components::CrumbCollectRadius};

#[derive(Bundle)]
pub struct PlayerBundle {
    speed: Speed,
    marker: Player,
    hp: Health,
    shooter_marker: Shooter,
    shooter: ShooterBundle,
    sprite: SpriteBundle,
    remove_on_reset: RemoveOnReset,
    collect_radius: CrumbCollectRadius,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        let start_pos = Transform::default();
        let sprite_size = Vec2::new(50., 50.);
        let player_hp = 30;
        Self {
            speed: Speed(125.),
            marker: Player,
            hp: Health(player_hp),
            shooter_marker: Shooter::Player,
            collect_radius: CrumbCollectRadius(200.),
            shooter: ShooterBundle {
                damage: Damage(5),
                reload_time: ReloadTime(time::Duration::from_secs_f32(0.25)),
                since_last_reload: ReloadStopwatch(
                    Stopwatch::new()
                        .tick(std::time::Duration::from_secs_f32(0.25))
                        .clone(),
                ),
            },
            remove_on_reset: RemoveOnReset,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(sprite_size),
                    ..default()
                },
                transform: start_pos,
                ..default()
            },
        }
    }
}
