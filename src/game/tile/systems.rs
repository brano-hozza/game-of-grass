use super::components::Tile;
use super::components::TileMap;
use super::components::Tiles;
use super::resources::TileTextures;
use super::TileType;
use super::TILE_WEIGHTS;
use crate::game::components::Point;
use crate::{TILE_SIZE, VISIBLE_HEIGHT, VISIBLE_WIDTH};
use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn spawn_tiles(
    mut commands: Commands,
    tile_sprites: Res<TileTextures>,
    // asset_server: Res<AssetServer>,
) {
    let mut game_map = TileMap {
        map: Vec::from([TileType::Grass; VISIBLE_WIDTH * VISIBLE_HEIGHT]),
        width: VISIBLE_WIDTH,
        height: VISIBLE_HEIGHT,
    };

    let mut rng = thread_rng();
    for x in 0..game_map.width {
        for y in 0..game_map.height {
            if x == 0 || y == 0 || x == game_map.width - 1 || y == game_map.height - 1 {
                game_map.set_tile(&Point::new(x as i32, y as i32), TileType::Water);
                continue;
            }
            if x == 1 || y == 1 || x == game_map.width - 2 || y == game_map.height - 2 {
                game_map.set_tile(&Point::new(x as i32, y as i32), TileType::Grass);
                continue;
            }
            let random_tile = TILE_WEIGHTS
                .choose_weighted(&mut rng, |item| item.1)
                .unwrap()
                .0;

            game_map.set_tile(&Point::new(x as i32, y as i32), random_tile);
        }
    }

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
