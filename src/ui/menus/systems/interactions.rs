use bevy::prelude::*;

use crate::{
    gameplay::player::bundles::PlayerBundle,
    ui::{
        menus::{
            components::{QuitButton, ResetButton, ResumeButton, StyledButton},
            styles::ButtonStyle,
        },
        score::{resources::PlayerScore, systems::layout::build_score_count},
    },
    PausedState, RemoveOnReset,
};

pub fn interact_styled_button<T: Component>(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StyledButton>, With<T>),
    >,
) {
    if let Ok((interaction, mut bg_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = ButtonStyle::press_bg_color().into();
            }
            Interaction::Hovered => {
                *bg_color = ButtonStyle::hover_bg_color().into();
            }
            Interaction::None => {
                *bg_color = ButtonStyle::bg_color().into();
            }
        }
    }
}

pub fn interact_with_resume_button(
    button_query: Query<&Interaction, (Changed<Interaction>, With<ResumeButton>)>,
    mut next_paused_state: ResMut<NextState<PausedState>>,
) {
    if let Ok(Interaction::Pressed) = button_query.get_single() {
        next_paused_state.set(PausedState::Running);
    }
}

pub fn interact_with_quit_button(
    button_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if let Ok(Interaction::Pressed) = button_query.get_single() {
        app_exit_events.send(bevy::app::AppExit);
    }
}

pub fn interact_with_reset_button(
    button_query: Query<&Interaction, (Changed<Interaction>, With<ResetButton>)>,
    mut next_paused_state: ResMut<NextState<PausedState>>,
    mut remove_query: Query<Entity, With<RemoveOnReset>>,
    mut score: ResMut<PlayerScore>,
    mut commands: Commands,
) {
    if let Ok(Interaction::Pressed) = button_query.get_single() {
        next_paused_state.set(PausedState::Running);
        remove_query.iter_mut().for_each(|ent| {
            commands.entity(ent).despawn_recursive();
        });

        *score = PlayerScore(0);
        build_score_count(&mut commands, &Res::from(score));
        commands.spawn(PlayerBundle::default());
    }
}
