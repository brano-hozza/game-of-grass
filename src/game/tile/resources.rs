use std::ops::Index;

use bevy::prelude::*;

use super::TileType;

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

impl Index<TileType> for TileSprites {
    type Output = Handle<Image>;

    fn index(&self, tile_type: TileType) -> &Self::Output {
        match tile_type {
            TileType::Grass => &self.grass,
            TileType::Tree => &self.tree,
            TileType::Water => &self.water,
            TileType::Rock => &self.rock,
            TileType::Chest => &self.chest,
        }
    }
}

#[derive(Resource)]
pub struct GameMap {
    pub map: Vec<Vec<TileType>>,
}

impl Default for GameMap {
    fn default() -> GameMap {
        let mut map = vec![vec![TileType::Grass; 20]; 20];
        // Add some trees
        map[5][5] = TileType::Tree;
        map[5][6] = TileType::Tree;
        map[5][7] = TileType::Tree;

        // Add some stones
        map[10][10] = TileType::Rock;
        map[10][11] = TileType::Rock;

        // Add some water
        map[15][15] = TileType::Water;
        map[15][16] = TileType::Water;

        // Add a chest
        map[18][18] = TileType::Chest;

        GameMap { map }
    }
}
