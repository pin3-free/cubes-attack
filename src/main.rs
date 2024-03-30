use bevy::{prelude::*, window::PrimaryWindow};
// use bevy_prng::WyRand;
// use bevy_rand::prelude::GlobalEntropy;
// use rand_core::RngCore;
use rand::prelude::*;

fn draw_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

#[derive(Bundle)]
struct PlayerBundle {
    speed: Speed,
    marker: Player,
    sprite: SpriteBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            speed: Speed(125.),
            marker: Player,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::new(50., 50.)),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Direction(Vec2);

#[derive(Component)]
struct BulletLifetimeTimer(Timer);

#[derive(Bundle)]
struct BulletBundle {
    speed: Speed,
    marker: Bullet,
    direction: Direction,
    lifetime: BulletLifetimeTimer,
    sprite: SpriteBundle,
}

impl Default for BulletBundle {
    fn default() -> Self {
        Self {
            speed: Speed(300.),
            marker: Bullet,
            direction: Direction(Vec2::new(1., 0.)),
            lifetime: BulletLifetimeTimer(Timer::from_seconds(20., TimerMode::Once)),
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

fn get_delta(direction: &Direction, speed: &Speed, time: &Res<Time>) -> Vec3 {
    Vec3::new(
        direction.0.x * speed.0 * time.delta_seconds(),
        direction.0.y * speed.0 * time.delta_seconds(),
        0.,
    )
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health(u16);

#[derive(Bundle)]
struct EnemyBundle {
    speed: Speed,
    hp: Health,
    marker: Enemy,
    sprite: SpriteBundle,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            speed: Speed(100.),
            hp: Health(10),
            marker: Enemy,
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
    fn spawn_at(transform: Transform) -> Self {
        EnemyBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(50., 50.)),
                    ..default()
                },
                transform,
                ..Default::default()
            },
            ..default()
        }
    }
}

#[derive(Event)]
struct ShootEvent {
    source: Vec2,
    target: Vec2,
}

fn draw_player(mut commands: Commands) {
    commands.spawn(PlayerBundle::default());
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameplaySet {
    Enemies,
    Player,
    Bullets,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum InputSet {
    Mouse,
    Keyboard,
}

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

fn enemy_spawner(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let new_delay = rng.gen_range(2..5);
        timer.0 = Timer::from_seconds(new_delay as f32, TimerMode::Once);

        let income_angle = rng.gen_range((0.)..std::f32::consts::TAU);
        let primary_window = q_windows.single();
        let distance = ((primary_window.width() / 2.).powf(2.)
            + (primary_window.height() / 2.).powf(2.))
        .sqrt();
        let mut position = Transform::from_xyz(distance, 0., 0.);
        position.rotate_around(Vec3::ZERO, Quat::from_rotation_z(income_angle));

        commands.spawn(EnemyBundle::spawn_at(position));
    }
}

fn move_enemies(
    time: Res<Time>,
    mut q_enemies: Query<(&mut Transform, &Speed), With<Enemy>>,
    q_player: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_tr = q_player.single().translation;
    q_enemies.iter_mut().for_each(|(mut enemy_tr, enemy_spd)| {
        let dir = get_direction(&player_tr.xy(), &enemy_tr.translation.xy());
        enemy_tr.translation += Vec3::new(
            dir.x * time.delta_seconds() * enemy_spd.0,
            dir.y * time.delta_seconds() * enemy_spd.0,
            0.,
        );

        let enemy_fwd = (enemy_tr.rotation * Vec3::Y).xy();
        let dir_player = (player_tr.xy() - enemy_tr.translation.xy()).normalize();
        let forward_dot_player = enemy_fwd.dot(dir_player);

        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
            ()
        }

        let enemy_right = (enemy_tr.rotation * Vec3::X).xy();
        let right_dot = enemy_right.dot(dir_player);
        let rot_sign = -f32::copysign(1.0, right_dot);
        let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos();

        let rotation_angle = rot_sign * (10. * time.delta_seconds()).min(max_angle);

        enemy_tr.rotate_z(rotation_angle);
    });
}

fn move_player(
    mut query: Query<(&mut Transform, &Speed), With<Player>>,
    mut ev_move: EventReader<PlayerMoveEvent>,
    time: Res<Time>,
) {
    let (mut transform, speed) = query.single_mut();
    let delta = time.delta_seconds() * speed.0;
    ev_move
        .read()
        .for_each(|PlayerMoveEvent(direction)| match direction {
            MoveDirection::Up => transform.translation.y += delta,
            MoveDirection::Down => transform.translation.y -= delta,
            MoveDirection::Left => transform.translation.x -= delta,
            MoveDirection::Right => transform.translation.x += delta,
        });
}

enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Event)]
struct PlayerMoveEvent(MoveDirection);

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut ev_move: EventWriter<PlayerMoveEvent>) {
    keys.get_pressed().into_iter().for_each(|k| match k {
        KeyCode::KeyW => {
            ev_move.send(PlayerMoveEvent(MoveDirection::Up));
        }
        KeyCode::KeyA => {
            ev_move.send(PlayerMoveEvent(MoveDirection::Left));
        }
        KeyCode::KeyS => {
            ev_move.send(PlayerMoveEvent(MoveDirection::Down));
        }
        KeyCode::KeyD => {
            ev_move.send(PlayerMoveEvent(MoveDirection::Right));
        }
        _ => {}
    });
}

fn move_bullets(
    mut commands: Commands,
    mut q_bullets: Query<
        (
            &mut Transform,
            &Direction,
            &Speed,
            &mut BulletLifetimeTimer,
            Entity,
        ),
        With<Bullet>,
    >,
    timer: Res<Time>,
) {
    q_bullets.iter_mut().for_each(
        |(mut bullet_tr, bullet_dir, bullet_spd, mut bullet_timer, bullet_entity)| {
            bullet_tr.translation += get_delta(bullet_dir, bullet_spd, &timer);

            bullet_timer.0.tick(timer.delta());
            if bullet_timer.0.finished() {
                commands.entity(bullet_entity).despawn()
            }
        },
    )
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

fn bullet_spawner(mut commands: Commands, mut ev_shoot: EventReader<ShootEvent>) {
    ev_shoot.read().for_each(|ShootEvent { source, target }| {
        commands.spawn(BulletBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10., 10.)),
                    color: Color::RED,
                    ..default()
                },
                transform: Transform::from_xyz(source.x, source.y, 0.),
                ..default()
            },
            direction: Direction(get_direction(&target, &source)),
            ..default()
        });
    })
}

fn mouse_input(
    q_player: Query<&Transform, With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut ev_shoot: EventWriter<ShootEvent>,
    clicks: Res<ButtonInput<MouseButton>>,
) {
    let player_tr = q_player.get_single().unwrap();
    let primary_window = q_windows.get_single().unwrap();
    clicks.get_just_pressed().into_iter().for_each(|k| match k {
        MouseButton::Left => {
            if let Some(position) = q_windows.single().cursor_position() {
                let actual_position = Vec2::new(
                    position.x - primary_window.width() / 2.0,
                    primary_window.height() / 2.0 - position.y,
                );
                ev_shoot.send(ShootEvent {
                    source: player_tr.translation.xy(),
                    target: actual_position,
                });

                // let mut bullet_dir = player_tr.clone();
            }
        }
        _ => {}
    })
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(3., TimerMode::Once)))
        .add_event::<ShootEvent>()
        .add_event::<PlayerMoveEvent>()
        .add_systems(Startup, (draw_camera, draw_player).chain())
        .add_systems(
            Update,
            (
                (mouse_input).in_set(InputSet::Mouse),
                (keyboard_input).in_set(InputSet::Keyboard),
                (enemy_spawner, move_enemies).in_set(GameplaySet::Enemies),
                (bullet_spawner, move_bullets).in_set(GameplaySet::Bullets),
                (move_player).in_set(GameplaySet::Player),
            ),
        )
        .run();
}
