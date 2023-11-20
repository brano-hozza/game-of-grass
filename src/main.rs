pub mod events;
mod systems;

pub mod game;
pub mod main_menu;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugins((GamePlugin, MainMenuPlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(
            Update,
            (transition_to_game_state, transition_to_main_menu_state),
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
