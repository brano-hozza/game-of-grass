use std::time::Duration;

use super::components::{CatComponent, CatMovementTimer};
use crate::{
    game::{
        components::{Point, Rotation},
        tile::{components::TileMap, TileType},
    },
    TILE_SIZE,
};
use bevy::{prelude::*, window::PrimaryWindow};

pub fn spawn_cat(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Spawning cat!");
    let texture_handle = asset_server.load("sprites/cat.png");
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(10.0 * TILE_SIZE, 10.0 * TILE_SIZE, 0.0),
            texture: texture_handle,
            ..default()
        },
        CatComponent {},
        CatMovementTimer(Timer::new(Duration::from_secs(1), TimerMode::Once)),
        Rotation::Down,
        Point { x: 10, y: 10 },
    ));
}

pub fn move_cat(
    mut query: Query<
        (
            &mut Transform,
            &mut CatMovementTimer,
            &mut Rotation,
            &mut Point,
        ),
        With<CatComponent>,
    >,
    map_query: Query<&TileMap>,
    time: Res<Time>,
) {
    for (mut transform, mut timer, mut rotation, mut coordinate) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            println!("Cat moved!");
            let random_rotation: Rotation = rand::random();

            if random_rotation != *rotation {
                *rotation = random_rotation.clone();
                timer.0.reset();
                return;
            }

            let mut direction = Vec3::ZERO;
            if random_rotation == Rotation::Left {
                direction += Vec3::new(-1.0, 0.0, 0.0);
            } else if random_rotation == Rotation::Right {
                direction += Vec3::new(1.0, 0.0, 0.0);
            } else if random_rotation == Rotation::Up {
                direction += Vec3::new(0.0, 1.0, 0.0);
            } else if random_rotation == Rotation::Down {
                direction += Vec3::new(0.0, -1.0, 0.0);
            }

            if direction.length() > 0.0 {
                direction = direction.normalize();
            }

            let new_coordination = coordinate.clone() + direction;

            if let Ok(map) = map_query.get_single() {
                if let Some(tile) = map.get_tile(&new_coordination) {
                    if tile == &TileType::Grass {
                        *coordinate += direction;
                        transform.translation += direction * TILE_SIZE;
                    }
                }
            }
            timer.0.reset();
        }
    }
}

pub fn confine_cat_movement(
    mut cat_query: Query<&mut Transform, With<CatComponent>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut cat_transform) = cat_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let x_min = 0.0;
        let x_max = window.width() - TILE_SIZE;
        let y_min = 0.0;
        let y_max = window.height() - TILE_SIZE;

        let mut translation = cat_transform.translation;

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

        cat_transform.translation = translation;
    }
}

pub fn despawn_cat(mut commands: Commands, query: Query<Entity, With<CatComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
