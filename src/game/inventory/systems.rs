use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::{game::player::components::Player, INVENTORY_WIDTH};

use super::{
    components::{Inventory, InventoryUI, ItemIndex},
    events::InventoryChangeEvent,
    resources::ItemSprites,
};

pub fn spawn_inventory(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // UI
    let mut main_component = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            ..default()
        },
        InventoryUI {},
    ));
    main_component.with_children(|parent| {
        // Right vertical fill (border)
        let mut panel = parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(INVENTORY_WIDTH as f32),
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            background_color: Color::rgb(0.65, 0.65, 0.65).into(),
            ..default()
        });
        panel.with_children(|parent| {
            // Right vertical fill (content)
            let mut panel_content = parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(Val::Px(2.)),
                    align_items: AlignItems::Start,
                    padding: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            });
            panel_content.with_children(|parent| {
                // Title
                parent.spawn((
                    TextBundle::from_section(
                        "Inventory",
                        TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            ..default()
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    }),
                    Label,
                ));

                // Inventory
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::FlexStart,
                            border: UiRect::all(Val::Px(2.)),
                            ..default()
                        },
                        border_color: Color::rgb(0.5, 0.5, 0.5).into(),
                        ..default()
                    },
                    Inventory::default(),
                    AccessibilityNode(NodeBuilder::new(Role::List)),
                ));
            });
        });
    });
}

pub fn despawn_inventory(
    mut commands: Commands,
    mut inventory_query: Query<Entity, With<InventoryUI>>,
) {
    if let Ok(inventory) = inventory_query.get_single_mut() {
        commands.entity(inventory).despawn_recursive();
    }
}

pub fn update_inventory_ui(
    mut commands: Commands,
    mut ev_invent_change: EventReader<InventoryChangeEvent>,
    player_query: Query<&Children, With<Player>>,
    mut player_children_query: Query<(&mut Handle<Image>, &mut Sprite)>,
    inventory_query: Query<(Entity, &Inventory)>,
    inv_items_query: Query<&Children, With<Inventory>>,
    item_sprites: Res<ItemSprites>,
    asset_server: Res<AssetServer>,
) {
    let (inventory_entity, inventory) = inventory_query.get_single().expect("Error: No inventory");
    for _ev in ev_invent_change.read() {
        // Update UI - remove all children and re-add
        if let Ok(children) = inv_items_query.get_single() {
            for child in children.iter() {
                commands.entity(inventory_entity).remove_children(&[*child]);
                commands.entity(*child).despawn_recursive();
            }
        }

        let mut inventory_ui = commands.entity(inventory_entity);

        inventory_ui.with_children(|parent| {
            // Add items

            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            for (index, item_type) in inventory.item_placement.iter().enumerate() {
                let item = inventory.get_item(item_type).expect("Error: Missing item");

                let mut item_box = parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexEnd,
                        border: UiRect::all(Val::Px(2.)),
                        width: Val::Px(60.),
                        height: Val::Px(60.),
                        padding: UiRect {
                            top: Val::Px(2.),
                            bottom: Val::Px(0.),
                            ..default()
                        },
                        ..default()
                    },
                    border_color: Color::rgb(0.5, 0.5, 0.5).into(),
                    background_color: Color::GRAY.into(),

                    ..default()
                });

                item_box.with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(50.),
                                height: Val::Px(60.),
                                ..default()
                            },
                            background_color: Color::WHITE.into(),
                            ..default()
                        },
                        UiImage::new(item_sprites[item_type].clone()),
                    ));
                });

                item_box.with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            format!("{} = {}", item_type, item.amount),
                            TextStyle {
                                font: font.clone(),
                                font_size: 10.,
                                color: if inventory.selected_index == index {
                                    Color::RED
                                } else {
                                    Color::BLACK
                                },
                                ..default()
                            },
                        ),
                        ItemIndex(index),
                        Label,
                    ));
                });
            }
        });

        let player_children = player_query
            .get_single()
            .expect("Error: No player children");
        for child in player_children.iter() {
            let (mut texture, mut sprite) = player_children_query
                .get_mut(*child)
                .expect("Error: Missing player child");
            // Display in hand, empty hand if 0 amount
            if inventory.selected_index >= inventory.item_placement.len() {
                *texture = Handle::default();
                *sprite = Sprite::default();
                continue;
            }
            let item_type = inventory.item_placement[inventory.selected_index];

            *texture = item_sprites[&item_type].clone();
            *sprite = Sprite {
                custom_size: Some(Vec2::new(8., 8.)),
                ..default()
            };
        }
    }
}

pub fn player_item_select(
    mut mouse_input: EventReader<MouseWheel>,
    player_query: Query<&Children, With<Player>>,
    mut player_children_query: Query<(&mut Handle<Image>, &mut Sprite)>,
    mut items_query: Query<(&mut ItemIndex, &mut Text)>,
    mut inventory_query: Query<&mut Inventory>,
    item_sprites: Res<ItemSprites>,
) {
    let mut inventory = inventory_query
        .get_single_mut()
        .expect("Error: No inventory");
    let inv_size = inventory.item_placement.len();
    if inv_size == 0 {
        return;
    }
    let max_index = inv_size - 1;
    for ev in mouse_input.read() {
        match ev.unit {
            MouseScrollUnit::Pixel => {
                if ev.y < 0.0 {
                    if inventory.selected_index + 1 > max_index {
                        inventory.selected_index = 0;
                    } else {
                        inventory.selected_index += 1;
                    }
                } else if ev.y > 0.0 {
                    if (inventory.selected_index as i8) - 1 < 0 {
                        inventory.selected_index = max_index;
                    } else {
                        inventory.selected_index -= 1;
                    }
                }
            }
            MouseScrollUnit::Line => {
                if ev.y < 0.0 {
                    if inventory.selected_index + 1 > max_index {
                        inventory.selected_index = 0;
                    } else {
                        inventory.selected_index += 1;
                    }
                } else if ev.y > 0.0 {
                    if (inventory.selected_index as i8) - 1 < 0 {
                        inventory.selected_index = max_index;
                    } else {
                        inventory.selected_index -= 1;
                    }
                }
            }
        }

        // Display in UI

        for (index, mut text) in items_query.iter_mut() {
            text.sections[0].style.color = if index.0 == inventory.selected_index {
                Color::RED
            } else {
                Color::BLACK
            };
        }
        let item_type = inventory.item_placement[inventory.selected_index];

        let player_children = player_query
            .get_single()
            .expect("Error: No player children");

        for child in player_children.iter() {
            if let Ok((mut texture, mut sprite)) = player_children_query.get_mut(*child) {
                *texture = item_sprites[&item_type].clone();
                *sprite = Sprite {
                    custom_size: Some(Vec2::new(8., 8.)),
                    ..default()
                };
            }
        }
    }
}
