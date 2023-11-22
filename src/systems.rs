use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{
    events::*, AppState, GAME_SCALE, INVENTORY_WIDTH, TILE_SIZE, VISIBLE_HEIGHT, VISIBLE_WIDTH,
};

pub fn spawn_camera(
    mut commands: Commands,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
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

    // UI
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(INVENTORY_WIDTH as f32),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn((
                                TextBundle::from_section(
                                    "Inventory",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 30.0,
                                        ..default()
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(5.)),
                                    ..default()
                                }),
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                            ));
                        });
                });
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

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.read() {
        println!("Your final score is: {}", event.score.to_string());
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
