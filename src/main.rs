use core::time;

mod components;
mod events;
mod resources;
mod systems;
mod ui;

use bevy::{prelude::*, time::Stopwatch};
use rand::{distributions::Standard, prelude::*};

use components::*;
use events::*;
use resources::*;
use systems::*;
use ui::pause_menu::PauseMenuPlugin;
use ui::score::ScorePlugin;

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

#[derive(Bundle)]
struct PlayerBundle {
    speed: Speed,
    marker: Player,
    hp: Health,
    shooter_marker: Shooter,
    shooter: ShooterBundle,
    sprite: SpriteBundle,
    remove_on_reset: RemoveOnReset,
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

#[derive(Bundle)]
struct BulletBundle {
    speed: Speed,
    marker: Bullet,
    direction: MyDirection,
    damage: Damage,
    lifetime: BulletLifetimeTimer,
    sprite: SpriteBundle,
    shooter: Shooter,
    remove_on_reset: RemoveOnReset,
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

fn get_delta(direction: &MyDirection, speed: &Speed, time: &Res<Time>) -> Vec3 {
    Vec3::new(
        direction.0.x * speed.0 * time.delta_seconds(),
        direction.0.y * speed.0 * time.delta_seconds(),
        0.,
    )
}

#[derive(Bundle, Clone)]
struct EnemyBundle {
    speed: Speed,
    marker: Enemy,
    shooter_marker: Shooter,
    hp: Health,
    sprite: SpriteBundle,
    remove_on_reset: RemoveOnReset,
    point_worth: PointWorth,
}

#[derive(Bundle)]
struct ShooterEnemyBundle {
    enemy: EnemyBundle,
    shooter: ShooterBundle,
}

impl ShooterEnemyBundle {
    fn with_transform(self, transform: Transform) -> Self {
        Self {
            enemy: self.enemy.with_transform(transform),
            shooter: self.shooter,
        }
    }
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
            },
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

impl EnemyBundle {
    fn with_transform(self, transform: Transform) -> Self {
        Self {
            sprite: SpriteBundle {
                transform,
                ..self.sprite
            },
            ..self
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
        .add_plugins((DefaultPlugins, PauseMenuPlugin, ScorePlugin))
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(3., TimerMode::Once)))
        .add_event::<ShootEvent>()
        .add_event::<PlayerMoveEvent>()
        .init_state::<PausedState>()
        .add_systems(Startup, (draw_camera, draw_player).chain())
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
                (
                    enemy_spawner,
                    move_enemies,
                    enemies_shoot,
                    get_enemy_collisions,
                )
                    .in_set(GameplaySet::Enemies),
                (
                    bullet_spawner,
                    move_bullets,
                    bullet_collision_processing,
                    stop_highlight,
                    dead_mark,
                    dead_cleanup,
                )
                    .in_set(GameplaySet::Bullets),
                (move_player).in_set(GameplaySet::Player),
                (push_processor, on_hit_highlight, invulnerable_tick).in_set(GameplaySet::Global),
            ),
        )
        .run();
}
