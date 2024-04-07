use bevy::prelude::*;

use self::{menus::GlobalMenuPlugin, score::ScorePlugin};

pub mod menus;
pub mod score;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GlobalMenuPlugin, ScorePlugin));
    }
}
