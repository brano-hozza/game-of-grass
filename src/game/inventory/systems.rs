use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::*,
};

use crate::INVENTORY_WIDTH;

use super::{
    components::{Inventory, Item},
    events::NewItemEvent,
    resources::ItemSprites,
};

pub fn create_inventory(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    item_sprites: Res<ItemSprites>,
) {
    let inventory = Inventory::default();
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // UI
    let mut main_component = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexEnd,
            ..default()
        },
        ..default()
    });
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
                let mut inventory_box = parent.spawn((
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
                    inventory.clone(),
                    AccessibilityNode(NodeBuilder::new(Role::List)),
                ));
                inventory_box.with_children(|parent| {
                    // List items
                    for (item_type, item) in inventory.items.iter() {
                        let mut item_box = parent.spawn((
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::FlexStart,
                                    justify_content: JustifyContent::FlexEnd,
                                    border: UiRect::all(Val::Px(2.)),
                                    width: Val::Px(50.),
                                    height: Val::Px(50.),
                                    padding: UiRect {
                                        left: Val::Px(5.),
                                        right: Val::Px(5.),
                                        top: Val::Px(2.),
                                        bottom: Val::Px(0.),
                                    },
                                    ..default()
                                },
                                border_color: Color::rgb(0.5, 0.5, 0.5).into(),
                                background_color: Color::WHITE.into(),

                                ..default()
                            },
                            UiImage::new(item_sprites[item_type].clone()),
                        ));
                        item_box.with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    format!("{} = {}", item_type, item.amount),
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 10.,
                                        color: Color::BLACK,
                                        ..default()
                                    },
                                ),
                                item.clone(),
                                Label,
                            ));
                        });
                    }
                });
            });
        });
    });
}

pub fn update_inventory_ui(
    mut ev_new_item: EventReader<NewItemEvent>,
    mut inventory_query: Query<(&mut Item, &mut Text)>,
) {
    for ev in ev_new_item.read() {
        if let Some((_, mut text)) = inventory_query
            .iter_mut()
            .find(|(i, _)| i.item_type == ev.item_type)
        {
            text.sections[0].value = format!("{} = {}", ev.item_type, ev.amount);
        }
    }
}
