use bevy::prelude::*;

use super::ItemType;

#[derive(Event)]
pub struct NewItemEvent {
    pub item_type: ItemType,
    pub amount: usize,
}
