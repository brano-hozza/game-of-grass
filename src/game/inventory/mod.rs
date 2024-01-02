use bevy::prelude::*;

use crate::AppState;

use self::{
    events::InventoryChangeEvent,
    resources::ItemSprites,
    systems::{despawn_inventory, player_item_select, spawn_inventory, update_inventory_ui},
};

use super::tile::TileType;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ItemType {
    None,
    Wood,
    Stone,
    Gold,
}

impl std::fmt::Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemType::None => write!(f, "None"),
            ItemType::Wood => write!(f, "Wood"),
            ItemType::Stone => write!(f, "Stone"),
            ItemType::Gold => write!(f, "Gold"),
        }
    }
}

impl Into<TileType> for ItemType {
    fn into(self) -> TileType {
        match self {
            ItemType::Wood => TileType::Tree,
            ItemType::Stone => TileType::Rock,
            _ => unreachable!(),
        }
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemSprites>()
            .add_systems(OnEnter(AppState::Game), spawn_inventory)
            .add_systems(OnExit(AppState::Game), despawn_inventory)
            .add_systems(Update, (update_inventory_ui, player_item_select))
            .add_event::<InventoryChangeEvent>();
    }
}
