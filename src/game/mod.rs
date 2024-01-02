use bevy::prelude::*;

mod components;
mod inventory;
mod player;
mod tile;

use crate::systems::exit_game;

use self::{inventory::InventoryPlugin, player::PlayerPlugin, tile::TilePlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, TilePlugin, InventoryPlugin))
            .add_systems(Update, exit_game);
    }
}
