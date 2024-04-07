use bevy::prelude::*;

use crate::ui::menus::level_up_menu::{
    bundles::UpgradeButtonBundle, components::UpgradeMenu, styles::UpgradeMenuStyle,
};

use rand::{seq::IteratorRandom, thread_rng};

#[derive(Clone, Copy, Debug)]
pub enum UpgradeVariant {
    Speed,
    Damage,
    Health,
    FireRate,
    ShotSpeed,
}

pub fn spawn_upgrade_menu(mut commands: Commands) {
    let mut rng = thread_rng();
    let upgrade_variants = [
        UpgradeVariant::Speed,
        UpgradeVariant::Damage,
        UpgradeVariant::Health,
        UpgradeVariant::FireRate,
        UpgradeVariant::ShotSpeed,
    ]
    .iter()
    .choose_multiple(&mut rng, 3);
    build_upgrade_menu(&mut commands, &upgrade_variants);
}

fn build_upgrade_menu(commands: &mut Commands, variants: &Vec<&UpgradeVariant>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: UpgradeMenuStyle::default().0,
                background_color: UpgradeMenuStyle::bg_color().into(),
                ..default()
            },
            UpgradeMenu,
        ))
        .with_children(|parent| {
            variants.iter().for_each(|var| {
                parent
                    .spawn(UpgradeButtonBundle::with_variant(**var))
                    .with_children(|bp| {
                        bp.spawn(UpgradeButtonBundle::get_text(&var));
                    });
            })
        })
        .id()
}
