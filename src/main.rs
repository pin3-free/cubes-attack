mod components;
mod events;
mod gameplay;
mod systems;
mod ui;

use bevy::prelude::*;

use events::*;
use gameplay::GameplayPlugin;

use ui::{menus::GlobalMenuPlugin, score::ScorePlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            GlobalMenuPlugin,
            ScorePlugin,
            GameplayPlugin,
        ))
        .run();
}
