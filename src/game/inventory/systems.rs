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
};

pub fn create_inventory(mut commands: Commands, asset_server: Res<AssetServer>) {
    let inventory = Inventory::default();
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
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Start,
                                padding: UiRect::all(Val::Px(5.)),
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

                            // Moving panel
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::FlexStart,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    inventory.clone(),
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                ))
                                .with_children(|parent| {
                                    // List items
                                    for (item_type, item) in inventory.clone().items.iter() {
                                        parent.spawn((
                                            TextBundle::from_section(
                                                format!("{} = {}", item_type, item.amount),
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 20.,
                                                    ..default()
                                                },
                                            ),
                                            item.clone(),
                                            Label,
                                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                        ));
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
