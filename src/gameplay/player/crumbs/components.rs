use bevy::prelude::*;

#[derive(Component)]
pub struct ExpGain(pub u32);

#[derive(Component)]
pub struct ExpCrumb;

#[derive(Component)]
pub struct CrumbCollectRadius(pub f32);
