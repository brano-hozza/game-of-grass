use bevy::prelude::*;

use super::components::MainMenu;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // UI
    let mut main_component = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },

            ..default()
        },
        MainMenu {},
    ));

    main_component.with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Game of Grass",
                TextStyle {
                    font: font.clone(),
                    font_size: 60.0,
                    ..default()
                },
            ),
            Label,
        ));
        parent.spawn((
            TextBundle::from_section(
                "Press \"G\" to start",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    ..default()
                },
            ),
            Label,
        ));
    });
}

pub fn despawn_main_menu(mut commands: Commands, mut menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(menu) = menu_query.get_single_mut() {
        commands.entity(menu).despawn_recursive();
    }
}
