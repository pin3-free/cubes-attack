use bevy::prelude::*;

use super::components::{Damage, ReloadStopwatch, ReloadTime, ShotSpeed};

#[derive(Bundle)]
pub struct ShooterBundle {
    pub damage: Damage,
    pub since_last_reload: ReloadStopwatch,
    pub reload_time: ReloadTime,
    pub shot_speed: ShotSpeed,
}
