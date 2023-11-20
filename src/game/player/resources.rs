use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerSprites {
    pub down: Handle<Image>,
    pub up: Handle<Image>,
    pub left: Handle<Image>,
    pub right: Handle<Image>,
}

impl FromWorld for PlayerSprites {
    fn from_world(world: &mut World) -> Self {
        // You have full access to anything in the ECS World from here.
        // For example, you can access (and mutate!) other resources:
        let asset_server = world.resource::<AssetServer>();
        PlayerSprites {
            down: asset_server.load("sprites/player_down.png"),
            up: asset_server.load("sprites/player_up.png"),
            left: asset_server.load("sprites/player_left.png"),
            right: asset_server.load("sprites/player_right.png"),
        }
    }
}
