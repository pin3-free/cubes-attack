use bevy::prelude::*;

use crate::{
    ui::score::{
        components::{ScoreCountNode, ScoreCountText},
        resources::PlayerScore,
    },
    RemoveOnReset,
};

pub fn spawn_score_count(mut commands: Commands, player_score: Res<PlayerScore>) {
    build_score_count(&mut commands, &player_score);
}

pub fn despawn_score_count(
    mut commands: Commands,
    query_score: Query<Entity, With<ScoreCountText>>,
) {
    if let Ok(entity) = query_score.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn build_score_count(commands: &mut Commands, player_score: &Res<PlayerScore>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::End,
                    width: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            ScoreCountNode,
            RemoveOnReset,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    format!("{score}", score = player_score.0),
                    TextStyle {
                        font_size: 32.,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ScoreCountText,
            ));
        })
        .id()
}
