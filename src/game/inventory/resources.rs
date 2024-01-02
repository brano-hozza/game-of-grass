use std::ops::Index;

use bevy::prelude::*;

use super::ItemType;

#[derive(Resource)]
pub struct ItemSprites {
    pub none: Handle<Image>,
    pub wood: Handle<Image>,
    pub stone: Handle<Image>,
    pub gold: Handle<Image>,
}

impl FromWorld for ItemSprites {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        ItemSprites {
            none: asset_server.load("sprites/items/none.png"),
            wood: asset_server.load("sprites/items/wood.png"),
            stone: asset_server.load("sprites/items/stone.png"),
            gold: asset_server.load("sprites/items/gold.png"),
        }
    }
}

impl Index<&ItemType> for ItemSprites {
    type Output = Handle<Image>;

    fn index(&self, tile_type: &ItemType) -> &Self::Output {
        match tile_type {
            ItemType::None => &self.none,
            ItemType::Wood => &self.wood,
            ItemType::Stone => &self.stone,
            ItemType::Gold => &self.gold,
        }
    }
}
