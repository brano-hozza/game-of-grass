use bevy::prelude::*;

use self::systems::create_inventory;

pub mod components;
pub mod systems;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_inventory);
    }
}
