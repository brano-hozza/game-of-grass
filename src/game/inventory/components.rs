use std::collections::HashMap;

use bevy::prelude::*;

use super::ItemType;

#[derive(Clone, Component, Debug)]
pub struct Item {
    pub item_type: ItemType,
    pub amount: usize,
}

#[derive(Clone, Component)]
pub struct ItemIndex(pub usize);

#[derive(Component, Clone, Debug)]
pub struct Inventory {
    pub items: HashMap<ItemType, Item>,
    pub item_placement: Vec<ItemType>,
    pub selected_index: usize,
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            items: HashMap::<ItemType, Item>::new(),
            item_placement: vec![],
            selected_index: 0,
        }
    }
}

impl Inventory {
    pub fn add_item(&mut self, item: Item) {
        if let Some(existing_item) = self.items.get_mut(&item.item_type) {
            existing_item.amount += item.amount;
        } else {
            self.items.insert(item.item_type.clone(), item.clone());
            self.item_placement.push(item.item_type.clone());
        }
    }

    pub fn remove_item(&mut self, item_type: &ItemType, amount: usize) {
        if let Some(existing_item) = self.items.get_mut(item_type) {
            existing_item.amount -= amount;

            if existing_item.amount == 0 {
                self.items.remove(item_type);
                self.item_placement.retain(|item| item != item_type);
                if self.selected_index + 1 > self.item_placement.len() {
                    self.selected_index = 0;
                }
            }
        }
    }

    pub fn get_item(&self, item_type: &ItemType) -> Option<&Item> {
        self.items.get(item_type)
    }
}

#[derive(Component)]
pub struct InventoryUI;
