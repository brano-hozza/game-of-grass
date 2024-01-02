use bevy::prelude::*;

mod bundles;
pub mod components;
mod events;
mod resources;
mod systems;

use systems::*;

use crate::AppState;

use self::resources::PlayerSprites;

use super::SimulationState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSprites>()
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(OnExit(AppState::Game), despawn_player)
            .add_systems(
                Update,
                (
                    player_movement,
                    confine_player_movement,
                    player_breaking,
                    player_item_select,
                )
                    .chain()
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
