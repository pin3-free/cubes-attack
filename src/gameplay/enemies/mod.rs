use bevy::prelude::*;

use crate::gameplay::states::GameState;

use self::{
    resources::EnemySpawnTimer,
    systems::{enemies_shoot, enemy_spawner, get_enemy_collisions, move_enemies},
};

pub mod bundles;
pub mod components;
pub mod resources;
pub mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(0.5, TimerMode::Once)))
            .add_systems(
                Update,
                (
                    enemy_spawner,
                    move_enemies,
                    enemies_shoot,
                    get_enemy_collisions,
                )
                    .run_if(in_state(GameState::Running)),
            );
    }
}
