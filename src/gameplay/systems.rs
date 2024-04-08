use crate::events::*;
use crate::gameplay::player::components::Player;
use crate::gameplay::player::events::PlayerMoveEvent;

use bevy::{prelude::*, window::PrimaryWindow};

use super::bundles::MainCameraBundle;
use super::components::{
    Damage, Dead, Health, HitBlinkTimer, Invulnerable, MainCamera, Pushed, ReloadStopwatch,
    ReloadTime, Shooter, ShotSpeed,
};
use super::get_delta;
use super::states::GameState;
use super::MoveDirection;

pub fn draw_camera(mut commands: Commands) {
    commands.spawn(MainCameraBundle::default());
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
            let delta = get_delta(&push_info.direction, &push_info.speed, &time);

            push_info.distance.0 -= delta.length();
            push_tr.translation += delta;

            if push_info.distance.0 <= 0. {
                commands.entity(push_entity).remove::<Pushed>();
            }
        });
}

pub fn dead_mark(
    q_health: Query<(&Health, Entity, Option<&Player>), Changed<Health>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    q_health
        .iter()
        .filter(|(Health(hp), _, _)| *hp <= 0)
        .for_each(|(_, entity, player_opt)| {
            commands.entity(entity).insert(Dead);

            if player_opt.is_some() {
                next_state.set(GameState::GameOver);
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
        (Changed<Health>, Without<Dead>),
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
    mut next_pause_state: ResMut<NextState<GameState>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    pause_state: Res<State<GameState>>,
    mut ev_move: EventWriter<PlayerMoveEvent>,
) {
    keys.get_just_pressed().for_each(|k| match k {
        KeyCode::Escape => match pause_state.get() {
            GameState::Running => next_pause_state.set(GameState::Paused),
            GameState::Paused => next_pause_state.set(GameState::Running),
            GameState::GameOver => {
                app_exit_events.send(bevy::app::AppExit);
            }
            GameState::Upgrading => {}
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

pub fn mouse_input(
    mut q_player: Query<
        (
            &Transform,
            &mut ReloadStopwatch,
            &ReloadTime,
            &Damage,
            &ShotSpeed,
        ),
        With<Player>,
    >,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    time: Res<Time>,
    mut ev_shoot: EventWriter<ShootEvent>,
    clicks: Res<ButtonInput<MouseButton>>,
) {
    if let (
        Ok((player_tr, mut reload_watch, reload_time, player_dmg, player_shot_speed)),
        Ok(primary_window),
        Ok((camera, camera_transform)),
    ) = (
        q_player.get_single_mut(),
        q_windows.get_single(),
        q_camera.get_single(),
    ) {
        clicks.get_pressed().for_each(|k| match k {
            MouseButton::Left => {
                if let Some(world_position) = primary_window
                    .cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())
                {
                    if reload_watch.0.tick(time.delta()).elapsed() >= reload_time.0 {
                        reload_watch.0.reset();
                        ev_shoot.send(ShootEvent {
                            source: player_tr.translation.xy(),
                            target: world_position,
                            damage: player_dmg.clone(),
                            shooter: Shooter::Player,
                            bullet_speed: player_shot_speed.clone(),
                        });
                    }
                }
            }
            _ => {}
        })
    }
}
