use bevy::prelude::*;

#[derive(Component)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Component)]
pub enum Rotation {
    Up,
    Down,
    Left,
    Right,
}
