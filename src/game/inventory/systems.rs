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
    components::{Inventory, InventoryUI, Item, ItemIndex},
    events::InventoryChangeEvent,
    resources::ItemSprites,
    ItemType,
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
    mut items_query: Query<(&mut Item, &mut Text)>,
    player_query: Query<(&Inventory, &Children), With<Player>>,
    mut player_children_query: Query<(&mut Handle<Image>, &mut Sprite)>,
    inventory_query: Query<Entity, With<Inventory>>,
    item_sprites: Res<ItemSprites>,
    asset_server: Res<AssetServer>,
) {
    for ev in ev_invent_change.read() {
        if let Some((_, mut text)) = items_query
            .iter_mut()
            .find(|(i, _)| i.item_type == ev.item_type)
        {
            text.sections[0].value = format!("{} = {}", ev.item_type, ev.amount);
        } else {
            println!("New item: {:?}", ev.item_type);
            // Add new item to UI
            if let Ok((inventory, _)) = player_query.get_single() {
                // New index for item
                let index = inventory.item_placement.len();
                let inventory_entity = inventory_query.get_single().expect("No inventory");
                let mut inventory_ui = commands.entity(inventory_entity);

                inventory_ui.with_children(|parent| {
                    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
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
                        background_color: Color::WHITE.into(),

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
                            UiImage::new(item_sprites[&ev.item_type].clone()),
                        ));
                    });

                    item_box.with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                format!("Empty"),
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
                            Item {
                                item_type: ev.item_type,
                                amount: 0,
                            },
                            Label,
                        ));
                    });
                });
            }
        }

        // Display in hand, empty hand if 0 amount
        if let Ok((inventory, player_children)) = player_query.get_single() {
            if inventory.item_placement[inventory.selected_index] != ev.item_type {
                continue;
            }
            for child in player_children.iter() {
                if let Ok((mut texture, mut sprite)) = player_children_query.get_mut(*child) {
                    *texture = match ev.amount {
                        0 => Handle::default(),
                        _ => item_sprites[&ev.item_type].clone(),
                    };
                    *sprite = match ev.amount {
                        0 => Sprite::default(),
                        _ => Sprite {
                            custom_size: Some(Vec2::new(8., 8.)),
                            ..default()
                        },
                    }
                }
            }
        }
    }
}

pub fn player_item_select(
    mut mouse_input: EventReader<MouseWheel>,
    mut player_query: Query<(&mut Inventory, &Children), With<Player>>,
    mut player_children_query: Query<(&mut Handle<Image>, &mut Sprite)>,
    mut items_query: Query<(&mut ItemIndex, &mut Text)>,
    item_sprites: Res<ItemSprites>,
) {
    if let Ok((mut inventory, player_children)) = player_query.get_single_mut() {
        for ev in mouse_input.read() {
            match ev.unit {
                MouseScrollUnit::Pixel => {
                    if ev.y < 0.0 {
                        if inventory.selected_index + 1 > 8 {
                            inventory.selected_index = 0;
                        } else {
                            inventory.selected_index += 1;
                        }
                    } else if ev.y > 0.0 {
                        if (inventory.selected_index as i8) - 1 < 0 {
                            inventory.selected_index = 8;
                        } else {
                            inventory.selected_index -= 1;
                        }
                    }
                }
                MouseScrollUnit::Line => {
                    if ev.y < 0.0 {
                        if inventory.selected_index + 1 > 8 {
                            inventory.selected_index = 0;
                        } else {
                            inventory.selected_index += 1;
                        }
                    } else if ev.y > 0.0 {
                        if (inventory.selected_index as i8) - 1 < 0 {
                            inventory.selected_index = 8;
                        } else {
                            inventory.selected_index -= 1;
                        }
                    }
                }
            }

            // Display in UI

            for (_, mut text) in items_query.iter_mut() {
                text.sections[0].style.color = Color::BLACK;
            }
            let item_type = inventory.item_placement[inventory.selected_index];
            let amount = if item_type == ItemType::None {
                0
            } else {
                inventory.get_item(&item_type).unwrap().amount
            };

            if let Some((_, mut text)) = items_query
                .iter_mut()
                .find(|(i, _)| i.0 == inventory.selected_index)
            {
                text.sections[0].style.color = Color::RED;
                if let Some(item) = inventory.items.get(&item_type) {
                    text.sections[0].value = format!("{} = {}", item_type, item.amount);
                } else {
                    text.sections[0].value = format!("Empty");
                }
            }

            for child in player_children.iter() {
                if let Ok((mut texture, mut sprite)) = player_children_query.get_mut(*child) {
                    *texture = if amount == 0 {
                        Handle::default()
                    } else {
                        item_sprites[&item_type].clone()
                    };
                    *sprite = if amount == 0 {
                        Sprite::default()
                    } else {
                        Sprite {
                            custom_size: Some(Vec2::new(8., 8.)),
                            ..default()
                        }
                    }
                }
            }
        }
    }
}
