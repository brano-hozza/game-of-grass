use bevy::prelude::*;

use crate::{VISIBLE_HEIGHT, VISIBLE_WIDTH};

use super::TileType;
#[derive(Component)]
pub struct Tile {}

#[derive(Component)]
pub struct Tiles {}

#[derive(Component)]
pub struct TileMap {
    pub map: [TileType; VISIBLE_HEIGHT * VISIBLE_WIDTH],
    pub width: usize,
    pub height: usize,
}

impl TileMap {
    pub fn get_tile(&self, x: usize, y: usize) -> &TileType {
        &self.map[x + y * self.width]
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut TileType> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&mut self.map[x + y * self.width])
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile_type: TileType) {
        self.map[x + y * self.width] = tile_type;
    }
}
