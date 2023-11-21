use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use systems::*;

use crate::AppState;

use self::resources::{GameMap, TileSprites};

#[derive(Clone, Copy)]
pub enum TileType {
    Grass,
    Tree,
    Water,
    Rock,
    Chest,
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileSprites>()
            .init_resource::<GameMap>()
            .add_systems(OnEnter(AppState::Game), spawn_tiles)
            .add_systems(OnExit(AppState::Game), despawn_tiles);
    }
}
