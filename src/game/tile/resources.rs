use std::ops::Index;

use bevy::prelude::*;

use super::TileType;

#[derive(Resource)]
pub struct TileSprites {
    pub grass: Handle<Image>,
    pub tree: Handle<Image>,
    pub water: Handle<Image>,
    pub rock: Handle<Image>,
    pub chest: Handle<Image>,
}

impl FromWorld for TileSprites {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        TileSprites {
            grass: asset_server.load("sprites/tiles/grass.png"),
            tree: asset_server.load("sprites/tiles/tree.png"),
            water: asset_server.load("sprites/tiles/water.png"),
            rock: asset_server.load("sprites/tiles/rock.png"),
            chest: asset_server.load("sprites/tiles/chest.png"),
        }
    }
}

impl Index<&TileType> for TileSprites {
    type Output = Handle<Image>;

    fn index(&self, tile_type: &TileType) -> &Self::Output {
        match tile_type {
            TileType::Grass => &self.grass,
            TileType::Tree => &self.tree,
            TileType::Water => &self.water,
            TileType::Rock => &self.rock,
            TileType::Chest => &self.chest,
        }
    }
}
