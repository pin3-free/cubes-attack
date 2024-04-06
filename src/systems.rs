use crate::components::*;
use crate::events::*;
use crate::gameplay::player::components::Player;
use crate::gameplay::player::events::PlayerMoveEvent;
use crate::{get_delta, get_direction, BulletBundle, MoveDirection, PausedState};

use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    window::PrimaryWindow,
};

pub fn draw_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn invulnerable_tick(
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

pub fn push_processor(
    mut q_pushed: Query<(&mut Transform, &mut Pushed, Entity)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    q_pushed
        .iter_mut()
        .for_each(|(mut push_tr, mut push_info, push_entity)| {
            let delta = crate::get_delta(&push_info.direction, &push_info.speed, &time);

            push_info.distance.0 -= delta.length();
            push_tr.translation += delta;

            if push_info.distance.0 <= 0. {
                commands.entity(push_entity).remove::<Pushed>();
            }
        });
}

pub fn bullet_collision_processing(
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

pub fn dead_mark(
    q_health: Query<(&Health, Entity, Option<&Player>), Changed<Health>>,
    mut next_state: ResMut<NextState<crate::PausedState>>,
    mut commands: Commands,
) {
    q_health
        .iter()
        .filter(|(Health(hp), _, _)| *hp <= 0)
        .for_each(|(_, entity, player_opt)| {
            commands.entity(entity).insert(Dead);

            if player_opt.is_some() {
                next_state.set(crate::PausedState::GameOver);
            }
        });
}

pub fn dead_cleanup(q_dead: Query<Entity, With<Dead>>, mut commands: Commands) {
    q_dead
        .iter()
        .for_each(|dead| commands.entity(dead).despawn());
}

pub fn on_hit_highlight(
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
            if health.is_changed() && !sprite.is_changed() {
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
        },
    );
}

pub fn stop_highlight(
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

pub fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_pause_state: ResMut<NextState<PausedState>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    pause_state: Res<State<PausedState>>,
    mut ev_move: EventWriter<PlayerMoveEvent>,
) {
    keys.get_just_pressed().for_each(|k| match k {
        KeyCode::Escape => match pause_state.get() {
            PausedState::Running => next_pause_state.set(PausedState::Paused),
            PausedState::Paused => next_pause_state.set(PausedState::Running),
            PausedState::GameOver => {
                app_exit_events.send(bevy::app::AppExit);
            }
        },
        _ => {}
    });
    keys.get_pressed().for_each(|k| match k {
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

pub fn move_bullets(
    mut commands: Commands,
    mut q_bullets: Query<
        (
            &mut Transform,
            &MyDirection,
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

pub fn bullet_spawner(mut commands: Commands, mut ev_shoot: EventReader<ShootEvent>) {
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
                direction: MyDirection(get_direction(target, source)),
                damage: damage.clone(),
                shooter: *shooter,
                ..default()
            });
        },
    )
}

pub fn mouse_input(
    mut q_player: Query<(&Transform, &mut ReloadStopwatch, &ReloadTime, &Damage), With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    mut ev_shoot: EventWriter<ShootEvent>,
    clicks: Res<ButtonInput<MouseButton>>,
) {
    let (player_tr, mut reload_watch, reload_time, player_dmg) = q_player.get_single_mut().unwrap();
    let primary_window = q_windows.get_single().unwrap();
    clicks.get_pressed().for_each(|k| match k {
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
