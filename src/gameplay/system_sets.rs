use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameplaySet {
    Enemies,
    Player,
    Bullets,
    Global,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputSet {
    Mouse,
    Keyboard,
}
