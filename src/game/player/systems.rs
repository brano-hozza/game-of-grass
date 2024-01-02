use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Player;
use super::resources::PlayerSprites;
use crate::game::components::{Point, Rotation};
use crate::game::inventory::components::{Inventory, Item};
use crate::game::inventory::events::InventoryChangeEvent;
use crate::game::inventory::ItemType;
use crate::game::tile::components::{Tile, TileMap};
use crate::game::tile::resources::TileTextures;
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
                texture: Handle::default(),
                transform: Transform::from_xyz(4., -1., 0.),
                ..default()
            });
        });
}

pub fn despawn_player(mut commands: Commands, mut player_query: Query<Entity, With<Player>>) {
    if let Ok(player) = player_query.get_single_mut() {
        commands.entity(player).despawn();
    }
}

fn moving_in_direction(keyboard_input: &Res<Input<KeyCode>>, rotation: &Rotation) -> bool {
    match rotation {
        Rotation::Left => {
            keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::A)
        }
        Rotation::Right => {
            keyboard_input.just_pressed(KeyCode::Right) || keyboard_input.just_pressed(KeyCode::D)
        }
        Rotation::Up => {
            keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::W)
        }
        Rotation::Down => {
            keyboard_input.just_pressed(KeyCode::Down) || keyboard_input.just_pressed(KeyCode::S)
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
        if *rotation != Rotation::Left && moving_in_direction(&keyboard_input, &Rotation::Left) {
            // println!("Player is moving left");
            *sprite = player_sprites.left.clone();
            *rotation = Rotation::Left;
            return;
        }

        if *rotation != Rotation::Right && moving_in_direction(&keyboard_input, &Rotation::Right) {
            // println!("Player is moving right");
            *sprite = player_sprites.right.clone();
            *rotation = Rotation::Right;
            return;
        }

        if *rotation != Rotation::Up && moving_in_direction(&keyboard_input, &Rotation::Up) {
            // println!("Player is moving up");
            *sprite = player_sprites.up.clone();
            *rotation = Rotation::Up;
            return;
        }

        if *rotation != Rotation::Down && moving_in_direction(&keyboard_input, &Rotation::Down) {
            // println!("Player is moving down");
            *sprite = player_sprites.down.clone();
            *rotation = Rotation::Down;
            return;
        }

        let mut direction = Vec3::ZERO;
        if moving_in_direction(&keyboard_input, &Rotation::Left) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        } else if moving_in_direction(&keyboard_input, &Rotation::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        } else if moving_in_direction(&keyboard_input, &Rotation::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        } else if moving_in_direction(&keyboard_input, &Rotation::Down) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        let new_coordination = coordinate.clone() + direction;

        // Transform to tile cords
        if let Ok(map) = map_query.get_single() {
            if let Some(tile) = map.get_tile(&new_coordination) {
                if tile == &TileType::Grass {
                    *coordinate += direction;
                    transform.translation += direction * TILE_SIZE;
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
    mut ev_new_item: EventWriter<InventoryChangeEvent>,
    tile_sprites: Res<TileTextures>,
) {
    if keyboard_input.pressed(KeyCode::E) {
        if let Ok((player_coordinates, rotation, mut inventory)) = player_query.get_single_mut() {
            if let Ok(mut game_map) = map_query.get_single_mut() {
                let target = player_coordinates + rotation;
                if let Some(tile) = game_map.get_tile_mut(&target) {
                    if tile == &TileType::Grass || tile == &TileType::Water {
                        return;
                    }

                    let mut sprite = tile_query
                        .iter_mut()
                        .find(|(_, point)| target == **point)
                        .unwrap()
                        .0;

                    let item_type: ItemType = tile.clone().into();
                    inventory.add_item(Item {
                        item_type,
                        amount: 1,
                    });

                    let item = inventory.get_item(&item_type).unwrap();

                    *tile = TileType::Grass;
                    *sprite = tile_sprites[&TileType::Grass].clone();

                    ev_new_item.send(InventoryChangeEvent {
                        item_type,
                        amount: item.amount,
                    })
                }
            }
        }
    }
}

pub fn try_place_item(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Point, &Rotation, &mut Inventory), With<Player>>,
    mut tile_query: Query<(&mut Handle<Image>, &Point), With<Tile>>,
    mut map_query: Query<&mut TileMap>,
    mut ev_invent_change: EventWriter<InventoryChangeEvent>,
    tile_textures: Res<TileTextures>,
) {
    if keyboard_input.pressed(KeyCode::Q) {
        if let Ok((player_coordinates, rotation, mut inventory)) = player_query.get_single_mut() {
            let item_type = inventory.item_placement[inventory.selected_index].clone();
            if item_type == ItemType::None || item_type == ItemType::Gold {
                return;
            }
            if let Ok(mut game_map) = map_query.get_single_mut() {
                let target = player_coordinates + rotation;
                if let Some(tile) = game_map.get_tile_mut(&target) {
                    if tile != &TileType::Grass {
                        return;
                    }
                    let mut texture = tile_query
                        .iter_mut()
                        .find(|(_, point)| target == **point)
                        .unwrap()
                        .0;

                    let item = inventory.get_item(&item_type).unwrap();
                    if item.amount > 0 {
                        inventory.remove_item(&item_type, 1);

                        let item = inventory.get_item(&item_type).unwrap();

                        *tile = item_type.into();
                        *texture = tile_textures[tile].clone();

                        ev_invent_change.send(InventoryChangeEvent {
                            item_type,
                            amount: item.amount,
                        })
                    }
                }
            }
        }
    }
}
