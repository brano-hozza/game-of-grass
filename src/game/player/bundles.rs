use bevy::prelude::*;

use crate::game::{components::Point, inventory::components::Inventory};

use super::components::Player;
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub coordinate: Point,
    pub sprite: Sprite,
    pub transform: Transform,
    pub inventory: Inventory,
}
