use bevy::prelude::*;

use crate::{game::components::Point, VISIBLE_HEIGHT, VISIBLE_WIDTH};

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
    fn is_safe(&self, point: &Point) -> bool {
        point.in_bounds(
            Point::zero(),
            Point::new(self.width as i32, self.height as i32),
        )
    }

    pub fn get_tile(&self, point: &Point) -> Option<&TileType> {
        if !self.is_safe(point) {
            return None;
        }
        Some(&self.map[(point.x + point.y * self.width as i32) as usize])
    }

    pub fn get_tile_mut(&mut self, point: &Point) -> Option<&mut TileType> {
        if !self.is_safe(point) {
            return None;
        }
        Some(&mut self.map[(point.x + point.y * self.width as i32) as usize])
    }

    pub fn set_tile(&mut self, point: &Point, tile_type: TileType) {
        if !self.is_safe(point) {
            return;
        }
        self.map[(point.x + point.y * self.width as i32) as usize] = tile_type;
    }
}
