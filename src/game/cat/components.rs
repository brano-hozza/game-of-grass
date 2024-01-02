use bevy::prelude::*;

#[derive(Component)]
pub struct CatComponent;

#[derive(Component)]
pub struct CatMovementTimer(pub Timer);
