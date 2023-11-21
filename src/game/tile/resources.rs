use std::ops::Index;

use bevy::prelude::*;

use crate::{VISIBLE_HEIGHT, VISIBLE_WIDTH};

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
        // You have full access to anything in the ECS World from here.
        // For example, you can access (and mutate!) other resources:
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

impl Index<TileType> for TileSprites {
    type Output = Handle<Image>;

    fn index(&self, tile_type: TileType) -> &Self::Output {
        match tile_type {
            TileType::Grass => &self.grass,
            TileType::Tree => &self.tree,
            TileType::Water => &self.water,
            TileType::Rock => &self.rock,
            TileType::Chest => &self.chest,
        }
    }
}

#[derive(Resource)]
pub struct GameMap {
    pub map: [TileType; VISIBLE_HEIGHT * VISIBLE_WIDTH],
    pub width: usize,
    pub height: usize,
}

impl GameMap {
    pub fn get_tile(&self, x: usize, y: usize) -> &TileType {
        &self.map[x + y * self.width]
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut TileType {
        &mut self.map[x + y * self.width]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile_type: TileType) {
        self.map[x + y * self.width] = tile_type;
    }
}

impl Default for GameMap {
    fn default() -> GameMap {
        let mut map = GameMap {
            map: [TileType::Grass; VISIBLE_WIDTH * VISIBLE_HEIGHT],
            width: VISIBLE_WIDTH,
            height: VISIBLE_HEIGHT,
        };
        // Add some trees
        map.set_tile(1, 1, TileType::Tree);
        map.set_tile(2, 1, TileType::Tree);
        map.set_tile(3, 1, TileType::Tree);

        map
    }
}
