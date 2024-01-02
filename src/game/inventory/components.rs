use std::collections::HashMap;

use bevy::prelude::*;

use super::ItemType;

#[derive(Clone, Component)]
pub struct Item {
    pub item_type: ItemType,
    pub amount: usize,
}

#[derive(Clone, Component)]
pub struct ItemIndex(pub usize);

#[derive(Component, Clone)]
pub struct Inventory {
    pub items: HashMap<ItemType, Item>,
    pub item_placement: Vec<ItemType>,
    pub selected_index: usize,
}

impl Default for Inventory {
    fn default() -> Self {
        let mut items = HashMap::<ItemType, Item>::new();
        items.insert(
            ItemType::Wood,
            Item {
                item_type: ItemType::Wood,
                amount: 0,
            },
        );

        items.insert(
            ItemType::Stone,
            Item {
                item_type: ItemType::Stone,
                amount: 0,
            },
        );

        items.insert(
            ItemType::Gold,
            Item {
                item_type: ItemType::Gold,
                amount: 0,
            },
        );

        Inventory {
            items,
            item_placement: vec![
                ItemType::Wood,
                ItemType::Stone,
                ItemType::Gold,
                ItemType::None,
                ItemType::None,
                ItemType::None,
                ItemType::None,
                ItemType::None,
                ItemType::None,
            ],
            selected_index: 0,
        }
    }
}

impl Inventory {
    pub fn add_item(&mut self, item: Item) {
        if let Some(existing_item) = self.items.get_mut(&item.item_type) {
            existing_item.amount += item.amount;
        } else {
            self.items.insert(item.item_type.clone(), item);
        }
    }

    pub fn remove_item(&mut self, item_type: &ItemType, amount: usize) {
        if let Some(existing_item) = self.items.get_mut(item_type) {
            existing_item.amount -= amount;
        }
    }

    pub fn get_item(&self, item_type: &ItemType) -> Option<&Item> {
        self.items.get(item_type)
    }
}
