use std::fmt::Formatter;

use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use systems::*;

use crate::AppState;

use self::resources::TileTextures;

use super::inventory::ItemType;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Grass,
    Tree,
    Water,
    Rock,
    Chest,
}

impl std::fmt::Display for TileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TileType::Grass => write!(f, "Grass"),
            TileType::Tree => write!(f, "Tree"),
            TileType::Water => write!(f, "Water"),
            TileType::Rock => write!(f, "Rock"),
            TileType::Chest => write!(f, "Chest"),
        }
    }
}

impl Into<ItemType> for TileType {
    fn into(self) -> ItemType {
        match self {
            TileType::Tree => ItemType::Wood,
            TileType::Rock => ItemType::Stone,
            TileType::Chest => ItemType::Gold,
            _ => unreachable!(),
        }
    }
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileTextures>()
            .add_systems(OnEnter(AppState::Game), spawn_tiles)
            // .add_systems(Update, update_tiles)
            .add_systems(OnExit(AppState::Game), despawn_tiles);
    }
}
