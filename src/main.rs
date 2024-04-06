mod components;
mod events;
mod gameplay;
mod systems;
mod ui;

use bevy::prelude::*;
use rand::{distributions::Standard, prelude::*};

use components::*;
use events::*;
use gameplay::{
    enemies::{
        bundles::{EnemyBundle, ShooterEnemyBundle},
        EnemyPlugin,
    },
    player::PlayerPlugin,
    projectiles::ProjectilesPlugin,
};
use systems::*;
use ui::{menus::GlobalMenuPlugin, score::ScorePlugin};

#[derive(Bundle)]
struct ShooterBundle {
    damage: Damage,
    since_last_reload: ReloadStopwatch,
    reload_time: ReloadTime,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum PausedState {
    #[default]
    Running,
    Paused,
    GameOver,
}

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
        match rng.gen_range(0..=1) {
            0 => EnemyVariant::Basic(EnemyBundle::default()),
            _ => EnemyVariant::Shooter(ShooterEnemyBundle::default()),
        }
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameplaySet {
    Enemies,
    Player,
    Bullets,
    Global,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum InputSet {
    Mouse,
    Keyboard,
}

enum MoveDirection {
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

#[derive(Bundle)]
struct GameOverScreenBundle {
    text: TextBundle,
    marker: GameOverScreen,
}

impl Default for GameOverScreenBundle {
    fn default() -> Self {
        Self {
            text: TextBundle {
                text: Text::from_section(
                    "Game Over",
                    TextStyle {
                        font_size: 100.,
                        color: Color::RED,
                        ..default()
                    },
                ),
                visibility: Visibility::Hidden,
                style: Style {
                    position_type: PositionType::Relative,
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                ..default()
            },
            marker: GameOverScreen,
        }
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            GlobalMenuPlugin,
            ScorePlugin,
            PlayerPlugin,
            EnemyPlugin,
            ProjectilesPlugin,
        ))
        .add_event::<ShootEvent>()
        .init_state::<PausedState>()
        .add_systems(Startup, (draw_camera).chain())
        .configure_sets(
            Update,
            (
                GameplaySet::Bullets,
                GameplaySet::Enemies,
                GameplaySet::Player,
                GameplaySet::Global,
                InputSet::Mouse,
            )
                .run_if(in_state(PausedState::Running)),
        )
        .add_systems(
            Update,
            (
                (mouse_input).in_set(InputSet::Mouse),
                (keyboard_input).in_set(InputSet::Keyboard),
                (stop_highlight, dead_mark, dead_cleanup).in_set(GameplaySet::Bullets),
                (push_processor, on_hit_highlight, invulnerable_tick).in_set(GameplaySet::Global),
            ),
        )
        .run();
}
