use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Player;
use super::resources::PlayerSprites;
use crate::game::components::{Point, Rotation};
use crate::game::inventory::components::{Inventory, Item};
use crate::game::inventory::events::NewItemEvent;
use crate::game::inventory::ItemType;
use crate::game::tile::components::{Tile, TileMap};
use crate::game::tile::resources::TileSprites;
use crate::game::tile::TileType;
use crate::TILE_SIZE;

pub fn spawn_player(mut commands: Commands, player_sprites: Res<PlayerSprites>) {
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.1),
                texture: player_sprites.down.clone(),
                ..default()
            },
            Player {},
            Rotation::Down,
            Point { x: 0, y: 0 },
            Inventory::default(),
        ))
        .with_children(|parent| {
            // Display selected item in hand
            parent.spawn(SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            });
        });
}

pub fn player_item_select(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Inventory, &Children), With<Player>>,
    mut children_query: Query<&mut Handle<Image>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((mut inventory, children)) = player_query.get_single_mut() {
        if let Some(_selected_item) = inventory.selected_item.clone() {
            if keyboard_input.just_pressed(KeyCode::O) {
                inventory.selected_item = None;
                for child in children.iter() {
                    if let Ok(mut item_sprite) = children_query.get_mut(*child) {
                        *item_sprite = Handle::default();
                    }
                }
            }
        } else {
            if keyboard_input.just_pressed(KeyCode::P) {
                inventory.selected_item = Some(ItemType::Wood);
                for child in children.iter() {
                    if let Ok(mut item_sprite) = children_query.get_mut(*child) {
                        *item_sprite = asset_server.load("sprites/tiles/wood.png");
                    }
                }
            }
        }
    }
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
            // println!("Player is moving left");
            *sprite = player_sprites.left.clone();
            *rotation = Rotation::Left;
            direction += Vec3::new(-1.0, 0.0, 0.0);
        } else if keyboard_input.just_pressed(KeyCode::Right)
            || keyboard_input.just_pressed(KeyCode::D)
        {
            // println!("Player is moving right");
            *sprite = player_sprites.right.clone();
            *rotation = Rotation::Right;
            direction += Vec3::new(1.0, 0.0, 0.0);
        } else if keyboard_input.just_pressed(KeyCode::Up)
            || keyboard_input.just_pressed(KeyCode::W)
        {
            // println!("Player is moving up");
            *sprite = player_sprites.up.clone();
            *rotation = Rotation::Up;
            direction += Vec3::new(0.0, 1.0, 0.0);
        } else if keyboard_input.just_pressed(KeyCode::Down)
            || keyboard_input.just_pressed(KeyCode::S)
        {
            // println!("Player is moving down");
            *sprite = player_sprites.down.clone();
            *rotation = Rotation::Down;
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        let new_coordination = coordinate.clone() + direction;

        // Transform to tile cords
        if let Ok(map) = map_query.get_single() {
            if let Some(tile) = map.get_tile(&new_coordination) {
                match tile {
                    TileType::Water | TileType::Tree | TileType::Chest | TileType::Rock => {}
                    TileType::Grass => {
                        if new_coordination.x >= 0 && new_coordination.y >= 0 {
                            coordinate.x += direction.x as i32;
                            coordinate.y += direction.y as i32;
                            transform.translation += direction * TILE_SIZE;
                        }
                    }
                }
            }
        }
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
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
    mut player_query: Query<(&Point, &Rotation, &mut Inventory), With<Player>>,
    mut tile_query: Query<(&mut Handle<Image>, &Point), With<Tile>>,
    mut map_query: Query<&mut TileMap>,
    mut ev_new_item: EventWriter<NewItemEvent>,
    tile_sprites: Res<TileSprites>,
) {
    if keyboard_input.pressed(KeyCode::E) {
        if let Ok((player_coordinates, rotation, mut inventory)) = player_query.get_single_mut() {
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
                if let Some(tile) = game_map.get_tile_mut(&target) {
                    match *tile {
                        TileType::Tree | TileType::Rock | TileType::Chest => {
                            if let Some((mut sprite, _)) =
                                tile_query.iter_mut().find(|(_, point)| target == **point)
                            {
                                let item_type = match tile {
                                    TileType::Tree => ItemType::Wood,
                                    TileType::Rock => ItemType::Stone,
                                    TileType::Chest => ItemType::Gold,
                                    _ => unreachable!(),
                                };
                                inventory.add_item(Item {
                                    item_type: item_type.clone(),
                                    amount: 1,
                                });

                                let item = inventory.get_item(&item_type).unwrap();

                                *tile = TileType::Grass;
                                *sprite = tile_sprites[&TileType::Grass].clone();

                                ev_new_item.send(NewItemEvent {
                                    item_type,
                                    amount: item.amount,
                                })
                            }
                        }
                        _ => {}
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
