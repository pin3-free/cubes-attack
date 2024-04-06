use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PausedState {
    #[default]
    Running,
    Paused,
    GameOver,
}
