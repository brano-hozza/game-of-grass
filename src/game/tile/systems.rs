use super::resources::TileSprites;
use super::{components::Tile, resources::GameMap};
use crate::TILE_SIZE;
use bevy::prelude::*;

pub fn spawn_tiles(
    mut commands: Commands,
    tile_sprites: Res<TileSprites>,
    game_map: Res<GameMap>,
    asset_server: Res<AssetServer>,
) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 10.0,
        color: Color::BLACK,
    };

    // Render tile map
    for x in 0..game_map.width {
        for y in 0..game_map.height {
            let tile = game_map.get_tile(x, y);
            let texture = tile_sprites[tile.clone()].clone();

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
                            (x + y * game_map.width).to_string(),
                            text_style.clone(),
                        ),
                        ..default()
                    },));
                });
        }
    }
}

pub fn despawn_tiles(mut commands: Commands, query: Query<Entity, With<Tile>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
