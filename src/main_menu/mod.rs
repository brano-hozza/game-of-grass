use bevy::prelude::*;

use crate::AppState;

use self::systems::{despawn_main_menu, spawn_main_menu};

mod components;
mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_main_menu)
            .add_systems(OnEnter(AppState::Game), despawn_main_menu)
            .add_systems(OnExit(AppState::Game), spawn_main_menu);
    }
}
