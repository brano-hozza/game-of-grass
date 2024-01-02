use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{AppState, GAME_SCALE, INVENTORY_WIDTH, TILE_SIZE, VISIBLE_HEIGHT, VISIBLE_WIDTH};

pub fn spawn_camera(
    mut commands: Commands,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = window_query.get_single_mut().unwrap();

    let win_width = ((VISIBLE_WIDTH + 1) * TILE_SIZE as usize) as f32;
    let win_height = ((VISIBLE_HEIGHT + 1) * TILE_SIZE as usize) as f32;

    window.resolution.set_physical_resolution(
        (win_width * GAME_SCALE) as u32 + INVENTORY_WIDTH,
        (win_height * GAME_SCALE) as u32,
    );

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            (win_width - TILE_SIZE * 2.0 + INVENTORY_WIDTH as f32 / 2.0) / 2.0,
            (win_height - TILE_SIZE * 2.0) / 2.0,
            999.,
        )
        .with_scale(Vec3 {
            x: 1.0 / GAME_SCALE,
            y: 1.0 / GAME_SCALE,
            z: 1.0,
        }),
        ..default()
    });
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if *app_state.get() != AppState::Game {
            next_app_state.set(AppState::Game);
            println!("Entered AppState::Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if *app_state.get() != AppState::MainMenu {
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
            println!("Entered AppState::MainMenu");
        }
    }
}
