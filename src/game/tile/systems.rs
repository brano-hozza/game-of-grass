use super::resources::TileSprites;
use super::TileType;
use super::{components::Tile, resources::GameMap};
use crate::{TILE_SCALE, TILE_SIZE};
use bevy::prelude::*;

pub fn spawn_tiles(mut commands: Commands, tile_sprites: Res<TileSprites>, game_map: Res<GameMap>) {
    // Render tile map
    for (y, row) in game_map.map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let texture = match tile {
                TileType::Grass => tile_sprites.grass.clone(),
                TileType::Tree => tile_sprites.tree.clone(),
                TileType::Water => tile_sprites.water.clone(),
                TileType::Rock => tile_sprites.rock.clone(),
                TileType::Chest => tile_sprites.chest.clone(),
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
