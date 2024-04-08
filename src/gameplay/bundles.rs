use bevy::prelude::*;

use super::components::{Damage, MainCamera, ReloadStopwatch, ReloadTime, ShotSpeed};

#[derive(Bundle)]
pub struct ShooterBundle {
    pub damage: Damage,
    pub since_last_reload: ReloadStopwatch,
    pub reload_time: ReloadTime,
    pub shot_speed: ShotSpeed,
}

#[derive(Bundle)]
pub struct MainCameraBundle {
    pub camera: Camera2dBundle,
    marker: MainCamera,
}

impl Default for MainCameraBundle {
    fn default() -> Self {
        Self {
            camera: Camera2dBundle::default(),
            marker: MainCamera,
        }
    }
}
