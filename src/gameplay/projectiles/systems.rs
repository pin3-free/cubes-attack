use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::{
    gameplay::{
        components::{Damage, Health, MyDirection, Shooter, Speed},
        get_delta, get_direction,
    },
    ShootEvent,
};

use super::{
    bundles::BulletBundle,
    components::{Bullet, BulletLifetimeTimer},
};

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
