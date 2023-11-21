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
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
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

pub const VISIBLE_WIDTH: usize = 20; // This is the number of tiles visible on the x-axis.
pub const VISIBLE_HEIGHT: usize = 20; // This is the number of tiles visible on the y-axis.

pub const TILE_SIZE: f32 = 16.0; // This is the basic tile size.
pub const GAME_SCALE: f32 = 2.0; // This is the scale of the tile sprite.
pub const MAP_PADDING: f32 = 2.0; // This is the padding around the map.
