use crate::{
    gameplay::components::{Health, PointWorth},
    ui::score::{components::ScoreCountText, events::ScoreUpEvent, resources::PlayerScore},
};
use bevy::prelude::*;

pub fn trigger_score_update(
    q_killed: Query<(&Health, &PointWorth), Changed<Health>>,
    mut ev_writer: EventWriter<ScoreUpEvent>,
) {
    q_killed
        .iter()
        .filter(|(Health(hp), _)| *hp <= 0)
        .for_each(|(_, points)| {
            ev_writer.send(ScoreUpEvent((*points).clone()));
        });
}

pub fn update_score(mut ev_score_up: EventReader<ScoreUpEvent>, mut score: ResMut<PlayerScore>) {
    ev_score_up
        .read()
        .for_each(|ScoreUpEvent(PointWorth(points))| score.0 += points);
}

pub fn update_score_text(
    mut q_score: Query<&mut Text, With<ScoreCountText>>,
    score: Res<PlayerScore>,
) {
    if score.is_changed() {
        if let Ok(mut text) = q_score.get_single_mut() {
            text.sections[0].value = format!("{score}", score = score.0);
        }
    }
}
