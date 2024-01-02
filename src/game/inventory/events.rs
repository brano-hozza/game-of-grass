use bevy::prelude::*;

use super::ItemType;

#[derive(Event)]
pub struct InventoryChangeEvent {
    pub item_type: ItemType,
    pub amount: usize,
}
