use bevy::prelude::*;

use crate::AppState;

use self::systems::{confine_cat_movement, despawn_cat, move_cat, spawn_cat};

mod components;
mod systems;
pub struct CatPlugin;

impl Plugin for CatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_cat)
            .add_systems(
                Update,
                (move_cat, confine_cat_movement)
                    .chain()
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), despawn_cat);
    }
}
