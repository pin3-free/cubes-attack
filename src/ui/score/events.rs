use bevy::prelude::*;

use crate::gameplay::components::PointWorth;

#[derive(Event)]
pub struct ScoreUpEvent(pub PointWorth);
