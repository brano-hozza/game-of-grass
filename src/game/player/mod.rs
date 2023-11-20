use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::AppState;

use super::SimulationState;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(OnExit(AppState::Game), despawn_player)
            .add_systems(
                Update,
                (player_movement, confine_player_movement)
                    .chain()
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
