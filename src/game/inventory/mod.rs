use bevy::prelude::*;

use self::{
    events::NewItemEvent,
    systems::{create_inventory, update_inventory_ui},
};

pub mod components;
pub mod events;
pub mod systems;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_inventory)
            .add_systems(Update, update_inventory_ui)
            .add_event::<NewItemEvent>();
    }
}
