use bevy::prelude::*;

#[derive(Event)]
pub struct BrokenTileEvent {
    pub position: (i32, i32),
}
