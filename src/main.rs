use core::time;

use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    time::Stopwatch,
    window::PrimaryWindow,
};
use rand::{
    distributions::{Distribution, Standard},
    prelude::*,
};

fn draw_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Player;

#[derive(Component, Clone)]
struct Damage(i32);

#[derive(Component, Clone)]
struct Speed(f32);

#[derive(Component, Clone)]
struct ReloadStopwatch(Stopwatch);

#[derive(Component)]
struct ReloadTime(time::Duration);

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
}

impl Default for PlayerBundle {
    fn default() -> Self {
        let start_pos = Transform::default();
        let sprite_size = Vec2::new(50., 50.);
        Self {
            speed: Speed(125.),
            marker: Player,
            hp: Health(30),
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

#[derive(Component)]
struct Distance(f32);

#[derive(Component)]
struct Pushed {
    distance: Distance,
    direction: Direction,
    speed: Speed,
}

#[derive(Component)]
struct Invulnerable {
    blink_timer: Timer,
    invuln_timer: Timer,
}

fn get_enemy_collisions(
    mut q_player: Query<
        (
            &Transform,
            &Sprite,
            &mut Health,
            Option<&Invulnerable>,
            Entity,
        ),
        With<Player>,
    >,
    q_enemies: Query<(&Transform, &Sprite), (Without<Player>, With<Enemy>)>,
    mut commands: Commands,
) {
    let (player_tr, player_sprite, mut player_hp, player_invulnerable, player_entity) =
        q_player.single_mut();
    let player_box = Aabb2d::new(
        player_tr.translation.xy(),
        player_sprite.custom_size.unwrap() * 0.5,
    );

    q_enemies.iter().for_each(|(e_tr, e_sprite)| {
        let enemy_box = Aabb2d::new(
            e_tr.translation.truncate(),
            e_sprite.custom_size.unwrap() * 0.5,
        );

        if player_box.intersects(&enemy_box) {
            let push_dir = get_direction(
                &player_tr.translation.truncate(),
                &e_tr.translation.truncate(),
            );

            if player_invulnerable.is_none() {
                player_hp.0 -= 5;
            }

            commands.entity(player_entity).insert(Pushed {
                distance: Distance(100.),
                direction: Direction(push_dir),
                speed: Speed(200.),
            });
        }
    })
}

fn invulnerable_tick(
    mut q_invulnerable: Query<(&mut Invulnerable, &mut Visibility, Entity)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    q_invulnerable
        .iter_mut()
        .for_each(|(mut inv, mut vis, entity)| {
            inv.invuln_timer.tick(time.delta());
            inv.blink_timer.tick(time.delta());

            if inv.blink_timer.just_finished() {
                match *vis {
                    Visibility::Hidden => {
                        *vis = Visibility::Visible;
                    }
                    Visibility::Visible => *vis = Visibility::Hidden,
                    _ => {
                        *vis = Visibility::Hidden;
                    }
                }
            }

            if inv.invuln_timer.finished() {
                *vis = Visibility::Visible;
                commands.entity(entity).remove::<Invulnerable>();
            }
        });
}

fn push_processor(
    mut q_pushed: Query<(&mut Transform, &mut Pushed, Entity)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    q_pushed
        .iter_mut()
        .for_each(|(mut push_tr, mut push_info, push_entity)| {
            let delta = get_delta(&push_info.direction, &push_info.speed, &time);

            push_info.distance.0 -= delta.length();
            push_tr.translation += delta;

            if push_info.distance.0 <= 0. {
                commands.entity(push_entity).remove::<Pushed>();
            }
        });
}

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Direction(Vec2);

#[derive(Component)]
struct BulletLifetimeTimer(Timer);

#[derive(Component, Clone, Copy)]
enum Shooter {
    Player,
    Enemy,
}

#[derive(Bundle)]
struct BulletBundle {
    speed: Speed,
    marker: Bullet,
    direction: Direction,
    damage: Damage,
    lifetime: BulletLifetimeTimer,
    sprite: SpriteBundle,
    shooter: Shooter,
}

impl Default for BulletBundle {
    fn default() -> Self {
        Self {
            speed: Speed(300.),
            marker: Bullet,
            direction: Direction(Vec2::new(1., 0.)),
            lifetime: BulletLifetimeTimer(Timer::from_seconds(20., TimerMode::Once)),
            shooter: Shooter::Player,
            damage: Damage(5),
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

fn bullet_collision_processing(
    mut q_bullets: Query<(&Transform, &Sprite, &Shooter, &Damage, Entity), With<Bullet>>,
    mut q_colliders: Query<(&Transform, &Sprite, &mut Health, &Shooter), Without<Bullet>>,
    mut commands: Commands,
) {
    q_bullets.iter_mut().for_each(
        |(bullet_tr, bullet_sprite, bullet_shooter, bullet_dmg, bullet_entity)| {
            let bullet_size = bullet_sprite.custom_size.unwrap();

            q_colliders.iter_mut().for_each(
                |(collider_tr, collider_sprite, mut collider_hp, entity_shooter)| {
                    let collider_size = collider_sprite.custom_size.unwrap();
                    if Aabb2d::new(collider_tr.translation.xy(), collider_size * 0.5)
                        .intersects(&Aabb2d::new(bullet_tr.translation.xy(), bullet_size * 0.5))
                    {
                        match (bullet_shooter, entity_shooter) {
                            (Shooter::Enemy, Shooter::Player)
                            | (Shooter::Player, Shooter::Enemy) => {
                                collider_hp.0 -= bullet_dmg.0;
                                commands.entity(bullet_entity).despawn();
                            }
                            (_, _) => {}
                        }
                    }
                },
            )
        },
    );
}

fn dead_cleanup(
    q_health: Query<(&Health, Entity, Option<&Player>)>,
    mut next_state: ResMut<NextState<PausedState>>,
    mut commands: Commands,
) {
    q_health
        .iter()
        .filter(|(Health(hp), _, _)| *hp <= 0)
        .for_each(|(_, entity, player_opt)| {
            commands.entity(entity).despawn();

            if player_opt.is_some() {
                next_state.set(PausedState::GameOver);
            }
        });
}

fn get_delta(direction: &Direction, speed: &Speed, time: &Res<Time>) -> Vec3 {
    Vec3::new(
        direction.0.x * speed.0 * time.delta_seconds(),
        direction.0.y * speed.0 * time.delta_seconds(),
        0.,
    )
}

#[derive(Component)]
struct HitBlinkTimer {
    return_to: Color,
    timer: Timer,
}

fn on_hit_highlight(
    mut hit_query: Query<
        (
            &mut Sprite,
            Ref<Health>,
            Option<&Player>,
            Option<&Invulnerable>,
            Entity,
        ),
        Changed<Health>,
    >,
    mut commands: Commands,
) {
    hit_query.iter_mut().for_each(
        |(mut sprite, health, opt_player, opt_invulnerable, entity)| {
            if health.is_changed() {
                if !sprite.is_changed() {
                    commands.entity(entity).insert(HitBlinkTimer {
                        return_to: sprite.color,
                        timer: Timer::from_seconds(0.05, TimerMode::Once),
                    });
                    sprite.color = Color::RED;

                    match (opt_player, opt_invulnerable) {
                        (Some(_), None) => {
                            commands.entity(entity).insert(Invulnerable {
                                blink_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                                invuln_timer: Timer::from_seconds(2., TimerMode::Once),
                            });
                        }
                        (_, _) => {}
                    }
                }
            }
        },
    );
}

fn stop_highlight(
    mut query: Query<(&mut Sprite, &mut HitBlinkTimer, Entity)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    query
        .iter_mut()
        .for_each(|(mut sprite, mut blink_timer, entity)| {
            if blink_timer.timer.tick(time.delta()).finished() {
                sprite.color = blink_timer.return_to;
                commands.entity(entity).remove::<HitBlinkTimer>();
            }
        })
}

#[derive(Component, Clone)]
struct Enemy;

#[derive(Component, Clone, Copy)]
struct Health(i32);

#[derive(Bundle, Clone)]
struct EnemyBundle {
    speed: Speed,
    marker: Enemy,
    shooter_marker: Shooter,
    hp: Health,
    sprite: SpriteBundle,
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
            marker: Enemy,
            shooter_marker: Shooter::Enemy,
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

#[derive(Event)]
struct ShootEvent {
    source: Vec2,
    target: Vec2,
    damage: Damage,
    shooter: Shooter,
}

fn draw_player(mut commands: Commands) {
    commands.spawn(PlayerBundle::default());
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
        let new_enemy = rand::random::<EnemyVariant>();

        match new_enemy {
            EnemyVariant::Basic(b) => {
                commands.spawn(b.with_transform(position));
            }
            EnemyVariant::Shooter(s) => {
                commands.spawn(s.with_transform(position));
            }
        }
    }
}

fn move_enemies(
    time: Res<Time>,
    mut q_enemies: Query<(&mut Transform, &Speed, &Sprite), With<Enemy>>,
    q_player: Query<(&Transform, &Sprite), (With<Player>, Without<Enemy>)>,
) {
    let (player_tr, player_sprite) = q_player.single();
    let player_box = Aabb2d::new(
        player_tr.translation.xy(),
        player_sprite.custom_size.unwrap() * 0.5,
    );
    q_enemies
        .iter_mut()
        .for_each(|(mut enemy_tr, enemy_spd, enemy_sprite)| {
            let dir = get_direction(&player_tr.translation.xy(), &enemy_tr.translation.xy());
            let delta = Vec3::new(
                dir.x * time.delta_seconds() * enemy_spd.0,
                dir.y * time.delta_seconds() * enemy_spd.0,
                0.,
            );
            let enemy_box = Aabb2d::new(
                enemy_tr.translation.xy(),
                enemy_sprite.custom_size.unwrap() * 0.5,
            );
            enemy_tr.translation += delta;
            if enemy_box.intersects(&player_box) {
                enemy_tr.translation -= delta;
            }

            let enemy_fwd = (enemy_tr.rotation * Vec3::Y).xy();
            let dir_player = (player_tr.translation.xy() - enemy_tr.translation.xy()).normalize();
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

fn enemies_shoot(
    time: Res<Time>,
    q_player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut q_enemies: Query<(&Transform, &mut ReloadStopwatch, &ReloadTime, &Damage), With<Enemy>>,
    mut ev_shoot: EventWriter<ShootEvent>,
) {
    let player_tr = q_player.single();
    q_enemies
        .iter_mut()
        .for_each(|(e_tr, mut e_reload, e_reload_time, e_damage)| {
            if e_reload.0.tick(time.delta()).elapsed() >= e_reload_time.0 {
                e_reload.0.reset();
                ev_shoot.send(ShootEvent {
                    source: e_tr.translation.xy(),
                    target: player_tr.translation.xy(),
                    damage: e_damage.clone(),
                    shooter: Shooter::Enemy,
                });
            }
        })
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

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_pause_state: ResMut<NextState<PausedState>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    pause_state: Res<State<PausedState>>,
    mut ev_move: EventWriter<PlayerMoveEvent>,
) {
    keys.get_just_pressed().into_iter().for_each(|k| match k {
        KeyCode::Escape => match pause_state.get() {
            PausedState::Running => next_pause_state.set(PausedState::Paused),
            PausedState::Paused => next_pause_state.set(PausedState::Running),
            PausedState::GameOver => {
                app_exit_events.send(bevy::app::AppExit);
            }
        },
        _ => {}
    });
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
    ev_shoot.read().for_each(
        |ShootEvent {
             source,
             target,
             damage,
             shooter,
         }| {
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
                damage: damage.clone(),
                shooter: *shooter,
                ..default()
            });
        },
    )
}

fn mouse_input(
    mut q_player: Query<(&Transform, &mut ReloadStopwatch, &ReloadTime, &Damage), With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    mut ev_shoot: EventWriter<ShootEvent>,
    clicks: Res<ButtonInput<MouseButton>>,
) {
    let (player_tr, mut reload_watch, reload_time, player_dmg) = q_player.get_single_mut().unwrap();
    let primary_window = q_windows.get_single().unwrap();
    clicks.get_pressed().into_iter().for_each(|k| match k {
        MouseButton::Left => {
            if let Some(position) = q_windows.single().cursor_position() {
                let actual_position = Vec2::new(
                    position.x - primary_window.width() / 2.0,
                    primary_window.height() / 2.0 - position.y,
                );
                if reload_watch.0.tick(time.delta()).elapsed() >= reload_time.0 {
                    reload_watch.0.reset();
                    ev_shoot.send(ShootEvent {
                        source: player_tr.translation.xy(),
                        target: actual_position,
                        damage: player_dmg.clone(),
                        shooter: Shooter::Player,
                    });
                }
            }
        }
        _ => {}
    })
}

#[derive(Component)]
struct GameOverScreen;

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

fn spawn_game_over_message(mut commands: Commands) {
    commands.spawn(GameOverScreenBundle::default());
}

fn reveal_game_over_screen(mut query: Query<&mut Visibility, With<GameOverScreen>>) {
    let mut game_over = query.single_mut();
    *game_over = Visibility::Visible;
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,))
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(3., TimerMode::Once)))
        .add_event::<ShootEvent>()
        .add_event::<PlayerMoveEvent>()
        .init_state::<PausedState>()
        .add_systems(
            Startup,
            (draw_camera, draw_player, spawn_game_over_message).chain(),
        )
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
                    dead_cleanup,
                )
                    .in_set(GameplaySet::Bullets)
                    .chain(),
                (move_player).in_set(GameplaySet::Player),
                (push_processor, on_hit_highlight, invulnerable_tick).in_set(GameplaySet::Global),
                reveal_game_over_screen.run_if(in_state(PausedState::GameOver)),
            ),
        )
        .run();
}
