use std::collections::HashMap;

use bevy::prelude::*;

use super::ItemType;

#[derive(Clone, Component)]
pub struct Item {
    pub item_type: ItemType,
    pub amount: usize,
}

#[derive(Component, Clone)]
pub struct Inventory {
    pub items: HashMap<ItemType, Item>,
    pub selected_item: Option<ItemType>,
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
            selected_item: None,
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

    pub fn get_item(&self, item_type: &ItemType) -> Option<&Item> {
        self.items.get(item_type)
    }
}
