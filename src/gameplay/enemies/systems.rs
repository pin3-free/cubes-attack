use crate::{
    gameplay::{
        components::{
            Damage, Distance, Health, Invulnerable, MyDirection, Pushed, ReloadStopwatch,
            ReloadTime, Shooter, Speed,
        },
        get_direction,
        player::components::Player,
        EnemyVariant,
    },
    ShootEvent,
};
use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    window::PrimaryWindow,
};
use rand::Rng;

use super::{components::Enemy, resources::EnemySpawnTimer};

pub fn enemy_spawner(
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

pub fn move_enemies(
    time: Res<Time>,
    mut q_enemies: Query<(&mut Transform, &Speed, &Sprite), With<Enemy>>,
    q_player: Query<(&Transform, &Sprite), (With<Player>, Without<Enemy>)>,
) {
    if let Ok((player_tr, player_sprite)) = q_player.get_single() {
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
                let dir_player =
                    (player_tr.translation.xy() - enemy_tr.translation.xy()).normalize();
                let forward_dot_player = enemy_fwd.dot(dir_player);

                // (forward_dot_player - 1.0).abs() < f32::EPSILON;

                let enemy_right = (enemy_tr.rotation * Vec3::X).xy();
                let right_dot = enemy_right.dot(dir_player);
                let rot_sign = -f32::copysign(1.0, right_dot);
                let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos();

                let rotation_angle = rot_sign * (10. * time.delta_seconds()).min(max_angle);

                enemy_tr.rotate_z(rotation_angle);
            });
    }
}

pub fn enemies_shoot(
    time: Res<Time>,
    q_player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut q_enemies: Query<(&Transform, &mut ReloadStopwatch, &ReloadTime, &Damage), With<Enemy>>,
    mut ev_shoot: EventWriter<ShootEvent>,
) {
    if let Ok(player_tr) = q_player.get_single() {
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
}

pub fn get_enemy_collisions(
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
    if let Ok((player_tr, player_sprite, mut player_hp, player_invulnerable, player_entity)) =
        q_player.get_single_mut()
    {
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
                    direction: MyDirection(push_dir),
                    speed: Speed(200.),
                });
            }
        })
    }
}
