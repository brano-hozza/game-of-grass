use bevy::prelude::*;

use crate::AppState;

use self::{
    events::NewItemEvent,
    resources::ItemSprites,
    systems::{create_inventory, update_inventory_ui},
};

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum ItemType {
    Wood,
    Stone,
    Gold,
}

impl std::fmt::Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemType::Wood => write!(f, "Wood"),
            ItemType::Stone => write!(f, "Stone"),
            ItemType::Gold => write!(f, "Gold"),
        }
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemSprites>()
            .add_systems(OnEnter(AppState::Game), create_inventory)
            .add_systems(Update, update_inventory_ui)
            .add_event::<NewItemEvent>();
    }
}
