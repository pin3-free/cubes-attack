use bevy::prelude::*;

use crate::PointWorth;

#[derive(Event)]
pub struct ScoreUpEvent(pub PointWorth);
