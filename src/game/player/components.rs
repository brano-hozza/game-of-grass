use bevy::{prelude::*, utils::HashMap};

#[derive(Component)]
pub struct Player {}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum ItemType {
    Wood,
    Stone,
    Gold,
}

pub struct Item {
    pub item_type: ItemType,
    pub amount: usize,
}

#[derive(Component, Default)]
pub struct Inventory {
    pub items: HashMap<ItemType, Item>,
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
