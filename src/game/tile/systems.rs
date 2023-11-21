use super::resources::TileSprites;
use super::{components::Tile, resources::GameMap};
use crate::{SCALED_TILE_SIZE, TILE_SCALE};
use bevy::prelude::*;

pub fn spawn_tiles(mut commands: Commands, tile_sprites: Res<TileSprites>, game_map: Res<GameMap>) {
    // Render tile map
    for (y, row) in game_map.map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let texture = tile_sprites[tile.clone()].clone();

            let real_x = x as f32 * SCALED_TILE_SIZE;
            let real_y = y as f32 * SCALED_TILE_SIZE;

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(real_x, real_y, 0.0)
                        .with_scale(Vec3::new(TILE_SCALE, TILE_SCALE, 1.0)),
                    texture,
                    ..default()
                },
                Tile {},
            ));
        }
    }
}

pub fn despawn_tiles(mut commands: Commands, query: Query<Entity, With<Tile>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
