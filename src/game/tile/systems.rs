use super::components::Tile;
use super::{TileType, TILE_MAP};
use crate::{TILE_SCALE, TILE_SIZE};
use bevy::prelude::*;

pub fn spawn_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Render tile map
    for (y, row) in TILE_MAP.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let texture = match tile {
                TileType::Grass => asset_server.load("sprites/grass.png"),
                TileType::Tree => asset_server.load("sprites/tree.png"),
                TileType::Water => asset_server.load("sprites/water.png"),
                TileType::Rock => asset_server.load("sprites/rock.png"),
                TileType::Chest => asset_server.load("sprites/chest.png"),
            };

            let real_x = x as f32 * TILE_SIZE * TILE_SCALE;
            let real_y = y as f32 * TILE_SIZE * TILE_SCALE;

            let mut sprite = SpriteBundle {
                transform: Transform::from_xyz(real_x, real_y, 0.0),
                texture,
                ..default()
            };

            sprite.transform.scale *= TILE_SCALE;

            commands.spawn((sprite, Tile {}));
        }
    }
}

pub fn despawn_tiles(mut commands: Commands, query: Query<Entity, With<Tile>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
