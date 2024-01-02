use super::components::Tile;
use super::components::TileMap;
use super::components::Tiles;
use super::resources::TileTextures;
use super::TileType;
use crate::game::components::Point;
use crate::{TILE_SIZE, VISIBLE_HEIGHT, VISIBLE_WIDTH};
use bevy::prelude::*;

pub fn spawn_tiles(
    mut commands: Commands,
    tile_sprites: Res<TileTextures>,
    // asset_server: Res<AssetServer>,
) {
    let mut game_map = TileMap {
        map: [TileType::Grass; VISIBLE_WIDTH * VISIBLE_HEIGHT],
        width: VISIBLE_WIDTH,
        height: VISIBLE_HEIGHT,
    };
    // Add some trees
    game_map.set_tile(&Point::new(1, 2), TileType::Tree);
    game_map.set_tile(&Point::new(2, 2), TileType::Tree);
    game_map.set_tile(&Point::new(3, 2), TileType::Tree);

    // Add some water
    game_map.set_tile(&Point::new(1, 1), TileType::Water);
    game_map.set_tile(&Point::new(2, 1), TileType::Water);
    game_map.set_tile(&Point::new(3, 1), TileType::Water);

    // Add some rocks
    game_map.set_tile(&Point::new(1, 3), TileType::Rock);
    game_map.set_tile(&Point::new(2, 3), TileType::Rock);

    // Add a chest
    game_map.set_tile(&Point::new(4, 4), TileType::Chest);

    // Render tile map
    let game_width = game_map.width;
    let game_height = game_map.height;
    for x in 0..game_width {
        for y in 0..game_height {
            let tile = game_map.get_tile(&Point::new(x as i32, y as i32)).unwrap();
            let texture = tile_sprites[tile].clone();
            let real_x = x as f32 * TILE_SIZE;
            let real_y = y as f32 * TILE_SIZE;

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(real_x, real_y, 0.0),
                    texture,
                    ..default()
                },
                Point::new(x as i32, y as i32),
                Tile {},
            ));
        }
    }
    commands.spawn((game_map, Tiles {}));
}

pub fn despawn_tiles(mut commands: Commands, query: Query<Entity, With<Tile>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
