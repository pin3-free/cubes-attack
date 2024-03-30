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

// impl Bullet {
//     fn spawn(item: Bullet, pos: Transform) -> (SpriteBundle, Self) {
//         (
//             SpriteBundle {
//                 sprite: Sprite {
//                     color: Color::RED,
//                     custom_size: Some(Vec2::new(10., 10.)),
//                     ..default()
//                 },
//                 transform: pos,
//                 ..default()
//             },
//             item,
//         )
//     }

fn get_delta(direction: &Direction, speed: &Speed, time: &Res<Time>) -> Vec3 {
    Vec3::new(
        direction.0.x * speed.0 * time.delta_seconds(),
        direction.0.y * speed.0 * time.delta_seconds(),
        0.,
    )
}
// }

#[derive(Component)]
struct Enemy {
    speed: f32,
    hp: u16,
}

impl Enemy {
    fn spawn(item: Enemy, pos: Transform) -> (SpriteBundle, Self) {
        (
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(50., 50.)),
                    ..default()
                },
                transform: pos,
                ..default()
            },
            item,
        )
    }
}

fn draw_player(mut commands: Commands) {
    commands.spawn(PlayerBundle::default());
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
        .sqrt()
            - 300.;
        let mut position = Transform::from_xyz(distance, 0., 0.);
        position.rotate_around(Vec3::ZERO, Quat::from_rotation_z(income_angle));

        commands.spawn(Enemy::spawn(
            Enemy {
                speed: 100.,
                hp: 10,
            },
            position,
        ));
    }
}

fn move_enemies(
    time: Res<Time>,
    mut q_enemies: Query<(&mut Transform, &Enemy), Without<Player>>,
    q_player: Query<&Transform, With<Player>>,
) {
    let player_tr = q_player.single().translation;
    q_enemies.iter_mut().for_each(|(mut enemy_tr, enemy)| {
        let dir = get_direction(&player_tr.xy(), &enemy_tr.translation.xy());
        enemy_tr.translation += Vec3::new(
            dir.x * time.delta_seconds() * enemy.speed,
            dir.y * time.delta_seconds() * enemy.speed,
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
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    query.iter_mut().for_each(|(mut transform, Speed(speed))| {
        let delta = time.delta_seconds() * speed;
        keys.get_pressed().into_iter().for_each(|k| match k {
            KeyCode::KeyW => transform.translation.y += delta,
            KeyCode::KeyA => transform.translation.x -= delta,
            KeyCode::KeyS => transform.translation.y -= delta,
            KeyCode::KeyD => transform.translation.x += delta,
            _ => {}
        })
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

fn shoot(
    q_player: Query<&Transform, With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
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
                // let mut bullet_dir = player_tr.clone();
                commands.spawn(BulletBundle {
                    sprite: SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(10., 10.)),
                            color: Color::RED,
                            ..default()
                        },
                        transform: player_tr.clone(),
                        ..default()
                    },
                    direction: Direction(get_direction(
                        &actual_position,
                        &player_tr.translation.xy(),
                    )),
                    ..default()
                });
            }
        }
        _ => {}
    })
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(3., TimerMode::Once)))
        .add_systems(Startup, (draw_camera, draw_player).chain())
        .add_systems(
            Update,
            (
                shoot,
                move_player,
                move_bullets,
                enemy_spawner,
                move_enemies,
            ),
        )
        .run();
}
