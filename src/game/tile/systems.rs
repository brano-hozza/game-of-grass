use super::components::Tile;
use super::components::TileMap;
use super::components::Tiles;
use super::resources::TileSprites;
use super::TileType;
use crate::{TILE_SIZE, VISIBLE_HEIGHT, VISIBLE_WIDTH};
use bevy::prelude::*;

pub fn spawn_tiles(
    mut commands: Commands,
    tile_sprites: Res<TileSprites>,
    asset_server: Res<AssetServer>,
) {
    let mut game_map = TileMap {
        map: [TileType::Grass; VISIBLE_WIDTH * VISIBLE_HEIGHT],
        width: VISIBLE_WIDTH,
        height: VISIBLE_HEIGHT,
    };
    // Add some trees
    game_map.set_tile(1, 1, TileType::Tree);
    game_map.set_tile(2, 1, TileType::Tree);
    game_map.set_tile(3, 1, TileType::Tree);

    // Add some water
    game_map.set_tile(1, 2, TileType::Water);
    game_map.set_tile(2, 2, TileType::Water);
    game_map.set_tile(3, 2, TileType::Water);

    // Add some rocks
    game_map.set_tile(5, 5, TileType::Rock);
    game_map.set_tile(6, 5, TileType::Rock);

    // Add a chest
    game_map.set_tile(5, 6, TileType::Chest);

    // Render tile map
    println!("Rendering tile map");
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 10.0,
        color: Color::BLACK,
    };

    let game_width = game_map.width;
    let game_height = game_map.height;
    for x in 0..game_width {
        for y in 0..game_height {
            let tile = game_map.get_tile(x, y);
            let texture = tile_sprites[tile].clone();
            let real_x = x as f32 * TILE_SIZE;
            let real_y = y as f32 * TILE_SIZE;

            commands
                .spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(real_x, real_y, 0.0),
                        texture,
                        ..default()
                    },
                    Tile {},
                ))
                .with_children(|parent| {
                    parent.spawn((Text2dBundle {
                        text: Text::from_section(
                            (x + y * game_width).to_string(),
                            text_style.clone(),
                        ),
                        ..default()
                    },));
                });
        }
    }
    commands.spawn((game_map, Tiles {}));
}

// pub fn update_tiles(mut query: Query<&mut TileMap, With<Tiles>>, tile_sprites: Res<TileSprites>) {
//     for (mut tile) in query.single_mut().unwrap().map.iter_mut() {
//         *sprite = tile_sprites[tile.].clone();
//     }
// }

pub fn despawn_tiles(mut commands: Commands, query: Query<Entity, With<Tile>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
