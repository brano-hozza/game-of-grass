use bevy::prelude::*;

use super::components::ItemType;

#[derive(Event)]
pub struct NewItemEvent {
    pub item_type: ItemType,
    pub amount: usize,
}
