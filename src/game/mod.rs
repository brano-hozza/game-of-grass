use bevy::prelude::*;

mod components;
mod inventory;
mod player;
mod systems;
mod tile;

use crate::{systems::exit_game, AppState};

use self::{inventory::InventoryPlugin, player::PlayerPlugin, tile::TilePlugin};
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, TilePlugin, InventoryPlugin))
            .add_state::<SimulationState>()
            .add_systems(Update, exit_game)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
