use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use systems::*;

use crate::AppState;

use self::resources::TileSprites;

pub enum TileType {
    Grass,
    Tree,
    Water,
    Rock,
    Chest,
}

pub const TILE_MAP: [[TileType; 5]; 5] = [
    [
        TileType::Grass,
        TileType::Grass,
        TileType::Grass,
        TileType::Grass,
        TileType::Grass,
    ],
    [
        TileType::Grass,
        TileType::Grass,
        TileType::Rock,
        TileType::Grass,
        TileType::Grass,
    ],
    [
        TileType::Grass,
        TileType::Grass,
        TileType::Tree,
        TileType::Grass,
        TileType::Grass,
    ],
    [
        TileType::Grass,
        TileType::Grass,
        TileType::Grass,
        TileType::Grass,
        TileType::Chest,
    ],
    [
        TileType::Grass,
        TileType::Grass,
        TileType::Grass,
        TileType::Water,
        TileType::Water,
    ],
];

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileSprites>()
            .add_systems(OnEnter(AppState::Game), spawn_tiles)
            .add_systems(OnExit(AppState::Game), despawn_tiles);
    }
}
