use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::ShootEvent;

use self::{
    components::{MyDirection, Speed},
    enemies::{
        bundles::{EnemyBundle, ShooterEnemyBundle},
        EnemyPlugin,
    },
    player::PlayerPlugin,
    projectiles::ProjectilesPlugin,
    states::GameState,
    system_sets::{GameplaySet, InputSet},
    systems::{
        dead_cleanup, dead_mark, draw_camera, fix_camera_to_player, invulnerable_tick,
        keyboard_input, mouse_input, on_hit_highlight, push_processor, stop_highlight,
    },
};

pub mod bundles;
pub mod components;
pub mod enemies;
pub mod player;
pub mod projectiles;
pub mod states;
pub mod system_sets;
pub mod systems;

fn get_delta(direction: &MyDirection, speed: &Speed, time: &Res<Time>) -> Vec3 {
    Vec3::new(
        direction.0.x * speed.0 * time.delta_seconds(),
        direction.0.y * speed.0 * time.delta_seconds(),
        0.,
    )
}

enum EnemyVariant {
    Basic(EnemyBundle),
    Shooter(ShooterEnemyBundle),
}

impl Distribution<EnemyVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemyVariant {
        match rng.gen_range(0..100) {
            0..=95 => EnemyVariant::Basic(EnemyBundle::default()),
            _ => EnemyVariant::Shooter(ShooterEnemyBundle::default()),
        }
    }
}

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

fn get_direction(src: &Vec2, target: &Vec2) -> Vec2 {
    let angle = (*target - *src).angle_between(Vec2::X);
    let mut dir = Quat::from_rotation_z(angle)
        .mul_vec3(-Vec3::X)
        .xy()
        .normalize();
    dir.y *= -1.;
    dir
}

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_event::<ShootEvent>()
            .configure_sets(
                Update,
                (
                    GameplaySet::Bullets,
                    GameplaySet::Enemies,
                    GameplaySet::Player,
                    GameplaySet::Global,
                    InputSet::Mouse,
                )
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(Startup, (draw_camera).chain())
            .add_systems(
                Update,
                (
                    (mouse_input).in_set(InputSet::Mouse),
                    (keyboard_input).in_set(InputSet::Keyboard),
                    (stop_highlight, dead_mark, dead_cleanup).in_set(GameplaySet::Bullets),
                    (
                        push_processor,
                        on_hit_highlight,
                        invulnerable_tick,
                        fix_camera_to_player,
                    )
                        .in_set(GameplaySet::Global),
                ),
            )
            .add_plugins((PlayerPlugin, EnemyPlugin, ProjectilesPlugin));
    }
}
