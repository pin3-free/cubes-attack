use bevy::prelude::*;

use crate::gameplay::components::RemoveOnReset;

use super::components::{ExpCrumb, ExpGain};

#[derive(Bundle)]
pub struct ExpCrumbBundle {
    sprite: SpriteBundle,
    gain: ExpGain,
    remove_on_reset: RemoveOnReset,
    crumb: ExpCrumb,
}

impl Default for ExpCrumbBundle {
    fn default() -> Self {
        Self::with_transform(Transform::from_xyz(0., 0., 0.))
    }
}

impl ExpCrumbBundle {
    pub fn with_transform(transform: Transform) -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::splat(10.)),
                    ..Default::default()
                },
                transform,
                ..default()
            },
            gain: ExpGain(5),
            remove_on_reset: RemoveOnReset,
            crumb: ExpCrumb,
        }
    }
}
