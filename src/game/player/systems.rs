use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Player;
use super::resources::PlayerSprites;
use crate::game::components::{Point, Rotation};
use crate::game::tile::components::TileMap;
use crate::game::tile::TileType;
use crate::{TILE_SIZE, VISIBLE_WIDTH};

pub fn spawn_player(mut commands: Commands, player_sprites: Res<PlayerSprites>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.1),
            texture: player_sprites.down.clone(),
            ..default()
        },
        Player {},
        Rotation::Down,
        Point { x: 0, y: 0 },
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<
        (
            &mut Transform,
            &mut Handle<Image>,
            &mut Rotation,
            &mut Point,
        ),
        With<Player>,
    >,
    player_sprites: Res<PlayerSprites>,
    map_query: Query<&TileMap>,
) {
    if let Ok((mut transform, mut sprite, mut rotation, mut coordinate)) =
        player_query.get_single_mut()
    {
        let mut direction = Vec3::ZERO;
        if keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::A) {
            println!("Player is moving left");
            *sprite = player_sprites.left.clone();
            *rotation = Rotation::Left;
            direction += Vec3::new(-1.0, 0.0, 0.0);
        } else if keyboard_input.just_pressed(KeyCode::Right)
            || keyboard_input.just_pressed(KeyCode::D)
        {
            println!("Player is moving right");
            *sprite = player_sprites.right.clone();
            *rotation = Rotation::Right;
            direction += Vec3::new(1.0, 0.0, 0.0);
        } else if keyboard_input.just_pressed(KeyCode::Up)
            || keyboard_input.just_pressed(KeyCode::W)
        {
            println!("Player is moving up");
            *sprite = player_sprites.up.clone();
            *rotation = Rotation::Up;
            direction += Vec3::new(0.0, 1.0, 0.0);
        } else if keyboard_input.just_pressed(KeyCode::Down)
            || keyboard_input.just_pressed(KeyCode::S)
        {
            println!("Player is moving down");
            *sprite = player_sprites.down.clone();
            *rotation = Rotation::Down;
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        let mut translation = transform.translation + direction * TILE_SIZE;

        // Transform to tile cords
        if let Ok(map) = map_query.get_single() {
            match map.get_tile(
                coordinate.x + direction.x as usize,
                coordinate.y + direction.y as usize,
            ) {
                TileType::Water | TileType::Tree | TileType::Chest | TileType::Rock => {
                    translation.x = transform.translation.x;
                    translation.y = transform.translation.y;
                }
                TileType::Grass => {
                    transform.translation = translation;
                    if direction.x > coordinate.x as f32 {
                        coordinate.x += 1;
                    } else if direction.x < coordinate.x as f32 {
                        coordinate.x -= 1;
                    }

                    if direction.y > coordinate.y as f32 {
                        coordinate.y += 1;
                    } else if direction.y < coordinate.y as f32 {
                        coordinate.y -= 1;
                    }
                }
            }
        }
    }
}

pub fn confine_player_movement(
    mut player_query: Query<(&mut Transform, &mut Point), With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((mut player_transform, mut coordinate)) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let x_min = 0.0;
        let x_max = window.width() - TILE_SIZE;
        let y_min = 0.0;
        let y_max = window.height() - TILE_SIZE;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the players y position.
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn player_breaking(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Point, &Rotation), With<Player>>,
    mut map_query: Query<&mut TileMap>,
) {
    if keyboard_input.pressed(KeyCode::E) {
        if let Ok((player_coordinates, rotation)) = player_query.get_single_mut() {
            if let Ok(mut game_map) = map_query.get_single_mut() {
                let target: Point = match rotation {
                    Rotation::Up => Point {
                        x: player_coordinates.x,
                        y: player_coordinates.y + 1,
                    },
                    Rotation::Down => Point {
                        x: player_coordinates.x,
                        y: player_coordinates.y - 1,
                    },
                    Rotation::Left => Point {
                        x: player_coordinates.x - 1,
                        y: player_coordinates.y,
                    },
                    Rotation::Right => Point {
                        x: player_coordinates.x + 1,
                        y: player_coordinates.y,
                    },
                };
                if let Some(tile) = game_map.get_tile_mut(target.x, target.y) {
                    println!("Player is breaking a tile at {} {}", target.x, target.y);
                    if *tile == TileType::Tree {
                        println!("Player is breaking a tree");
                        *tile = TileType::Grass;
                    }
                }
            }
        }
    }
}

pub fn despawn_player(mut commands: Commands, mut player_query: Query<Entity, With<Player>>) {
    if let Ok(player) = player_query.get_single_mut() {
        commands.entity(player).despawn();
    }
}
