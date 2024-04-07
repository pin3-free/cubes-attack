use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::gameplay::{
    components::Health,
    enemies::components::Enemy,
    player::{
        components::{Player, PlayerLevel},
        resources::PlayerExperience,
    },
    states::GameState,
};

use bevy::math::bounding::BoundingCircle;

use super::{
    bundles::ExpCrumbBundle,
    components::{CrumbCollectRadius, ExpCrumb, ExpGain},
    events::LevelUpEvent,
};

pub fn drop_crumbs(
    q_dead: Query<(&Transform, &Health), (With<Enemy>, Changed<Health>)>,
    mut commands: Commands,
) {
    q_dead.iter().for_each(|(tr, health)| {
        if health.0 <= 0 {
            commands.spawn(ExpCrumbBundle::with_transform(tr.clone()));
        }
    })
}

pub fn collect_crumbs(
    q_player: Query<(&Transform, &CrumbCollectRadius, &PlayerLevel), With<Player>>,
    q_crumbs: Query<(&Transform, &ExpGain, &Sprite, Entity), With<ExpCrumb>>,
    mut exp: ResMut<PlayerExperience>,
    mut lvl_events: EventWriter<LevelUpEvent>,
    mut commands: Commands,
) {
    if let Ok((player_tr, player_radius, player_level)) = q_player.get_single() {
        let player_radius = BoundingCircle::new(player_tr.translation.xy(), player_radius.0);

        q_crumbs
            .iter()
            .for_each(|(crumb_tr, exp_gain, sprite, entity)| {
                let crumb_box = Aabb2d::new(
                    crumb_tr.translation.truncate(),
                    sprite.custom_size.unwrap() * 0.5,
                );

                if player_radius.intersects(&crumb_box) {
                    (*exp).0 += exp_gain.0;
                    commands.entity(entity).despawn();
                }

                if player_level.next_level_delta.0 <= exp.0 {
                    lvl_events.send(LevelUpEvent);
                }
            })
    }
}

pub fn level_up(
    mut lvl_events: EventReader<LevelUpEvent>,
    mut q_player: Query<&mut PlayerLevel, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut player_exp: ResMut<PlayerExperience>,
) {
    if let Ok(mut level) = q_player.get_single_mut() {
        if !lvl_events.is_empty() {
            lvl_events.read().take(1).for_each(|_| {
                game_state.set(GameState::Upgrading);

                level.level += 1;
                level.next_level_delta.0 += 5 * level.next_level_delta.0.ilog2();
                player_exp.0 = 0;
            });
        }
    }
}
