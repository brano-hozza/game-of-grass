use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Player;
use super::resources::PlayerSprites;
use super::PLAYER_SIZE;
use crate::TILE_SCALE;
use crate::TILE_SIZE;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_sprites: Res<PlayerSprites>,
) {
    let window = window_query.get_single().unwrap();

    let x = window.width() / 2.0;
    let y = window.height() / 2.0;

    let mut sprite = SpriteBundle {
        transform: Transform::from_xyz(x, y, 1.0),
        texture: player_sprites.down.clone(),
        ..default()
    };

    sprite.transform.scale *= TILE_SCALE;

    commands.spawn((sprite, Player {}));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Handle<Image>), With<Player>>,
    player_sprites: Res<PlayerSprites>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Left, KeyCode::Right, KeyCode::Up, KeyCode::Down])
    {
        if let Ok((mut transform, mut sprite)) = player_query.get_single_mut() {
            let mut direction = Vec3::ZERO;
            if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
                println!("Left");
                *sprite = player_sprites.left.clone();
                direction += Vec3::new(-1.0, 0.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
                println!("Right");
                *sprite = player_sprites.right.clone();
                direction += Vec3::new(1.0, 0.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
                println!("Up");
                *sprite = player_sprites.up.clone();
                direction += Vec3::new(0.0, 1.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
                println!("Down");
                *sprite = player_sprites.down.clone();
                direction += Vec3::new(0.0, -1.0, 0.0);
            }

            if direction.length() > 0.0 {
                direction = direction.normalize();
            }

            transform.translation += direction * TILE_SIZE * TILE_SCALE;
        }
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0; // 32.0
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

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

pub fn despawn_player(mut commands: Commands, mut player_query: Query<Entity, With<Player>>) {
    if let Ok(player) = player_query.get_single_mut() {
        commands.entity(player).despawn();
    }
}
