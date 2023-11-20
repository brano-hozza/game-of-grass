use bevy::prelude::*;

#[derive(Resource)]
pub struct TileSprites {
    pub grass: Handle<Image>,
    pub tree: Handle<Image>,
    pub water: Handle<Image>,
    pub rock: Handle<Image>,
    pub chest: Handle<Image>,
}

impl FromWorld for TileSprites {
    fn from_world(world: &mut World) -> Self {
        // You have full access to anything in the ECS World from here.
        // For example, you can access (and mutate!) other resources:
        let asset_server = world.resource::<AssetServer>();
        TileSprites {
            grass: asset_server.load("sprites/tiles/grass.png"),
            tree: asset_server.load("sprites/tiles/tree.png"),
            water: asset_server.load("sprites/tiles/water.png"),
            rock: asset_server.load("sprites/tiles/rock.png"),
            chest: asset_server.load("sprites/tiles/chest.png"),
        }
    }
}
