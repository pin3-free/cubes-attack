mod components;
mod events;
mod gameplay;
mod systems;
mod ui;

use bevy::prelude::*;

use events::*;
use gameplay::GameplayPlugin;

use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin, GameplayPlugin))
        .run();
}
