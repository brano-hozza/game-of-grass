use std::ops::Index;

use bevy::prelude::*;

use super::ItemType;

#[derive(Resource)]
pub struct ItemSprites {
    pub wood: Handle<Image>,
    pub stone: Handle<Image>,
    pub gold: Handle<Image>,
}

impl FromWorld for ItemSprites {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        ItemSprites {
            wood: asset_server.load("sprites/tiles/tree.png"),
            stone: asset_server.load("sprites/tiles/rock.png"),
            gold: asset_server.load("sprites/tiles/chest.png"),
        }
    }
}

impl Index<&ItemType> for ItemSprites {
    type Output = Handle<Image>;

    fn index(&self, tile_type: &ItemType) -> &Self::Output {
        match tile_type {
            ItemType::Wood => &self.wood,
            ItemType::Stone => &self.stone,
            ItemType::Gold => &self.gold,
        }
    }
}
