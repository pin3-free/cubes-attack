use bevy::prelude::*;

use super::crumbs::components::ExpGain;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerLevel {
    pub level: u32,
    pub next_level_delta: ExpGain,
}
