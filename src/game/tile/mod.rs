use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::AppState;

enum TileType {
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
        app.add_systems(OnEnter(AppState::Game), spawn_tiles)
            .add_systems(OnExit(AppState::Game), despawn_tiles);
    }
}
