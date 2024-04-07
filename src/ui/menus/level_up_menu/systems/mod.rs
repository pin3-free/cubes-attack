use bevy::prelude::*;

use crate::gameplay::{
    components::{Damage, Health, ReloadTime, ShotSpeed, Speed},
    player::components::Player,
    states::GameState,
};

use self::layout::UpgradeVariant;

use super::{
    components::{UpgradeButton, UpgradeVariantComponent},
    events::UpgradeStatEvent,
};

pub mod interactions;
pub mod layout;

pub fn interact_upgrade_button(
    button_query: Query<
        (&Interaction, &UpgradeVariantComponent),
        (With<UpgradeButton>, Changed<Interaction>),
    >,
    mut ev_writer: EventWriter<UpgradeStatEvent>,
) {
    button_query
        .iter()
        .filter(|(interaction, _)| **interaction == Interaction::Pressed)
        .for_each(|(_, var)| {
            ev_writer.send(UpgradeStatEvent { variant: var.0 });
        });
}

pub fn process_upgrade_event(
    mut ev_reader: EventReader<UpgradeStatEvent>,
    mut upgrade_query: Query<
        (
            &mut Speed,
            &mut ShotSpeed,
            &mut Damage,
            &mut Health,
            &mut ReloadTime,
        ),
        With<Player>,
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok((mut speed, mut shotspeed, mut damage, mut health, mut reload_time)) =
        upgrade_query.get_single_mut()
    {
        ev_reader.read().for_each(|UpgradeStatEvent { variant }| {
            match variant {
                UpgradeVariant::Speed => {
                    speed.0 *= 1.1;
                }
                UpgradeVariant::Damage => {
                    damage.0 += 5;
                }
                UpgradeVariant::Health => {
                    health.0 += 10;
                    dbg!(format!("New health: {}", health.0));
                }
                UpgradeVariant::FireRate => {
                    reload_time.0 = reload_time.0.mul_f32(0.8);
                    dbg!(format!("New reload_time: {}", reload_time.0.as_secs_f32()));
                }
                UpgradeVariant::ShotSpeed => {
                    shotspeed.0 *= 1.1;
                    dbg!(format!("New shot_speed: {}", shotspeed.0));
                }
            }
            next_state.set(GameState::Running);
        });
    }
}
