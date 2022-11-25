pub mod height_map;

// Standard Lib Imports
use std::cmp::{max,min};

// Bevy Imports
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_mod_picking::PickableBundle;

use crate::plugins::world_3d::config::{HEX_CIRCUMRADIUS, HEX_GRID_RADIUS};
use height_map::{HeightMap, PerlinStep};

pub struct HexPlugin;

impl Plugin for HexPlugin {
    fn build(&self, app: &mut App) {
        app
        // .insert_resource(HeightMap::new_rand(1, 10, Some(42)))
        .insert_resource(HeightMap::new_perlin(vec![
            PerlinStep::new(0.1, 0.05, 20.)
        ], Some(42)))
        .add_startup_system(HexGrid::spawn);
    }
}

#[derive(Component, Inspectable, Debug, Copy, Clone)]
pub struct HexCoord(pub i32, pub i32);

impl HexCoord {
    pub fn to_world(&self) -> Vec3 {
        let x = HEX_CIRCUMRADIUS * f32::sqrt(3.0) * ((self.0 as f32) + (self.1 as f32) / 2.0);
        let y = HEX_CIRCUMRADIUS * (3.0/2.0) * (self.1 as f32);
        return Vec3 { x: x, y: 0.0, z: y };
    }

    pub fn to_bytes(self) -> [u8; 8] {
        let x:[u8; 4] = self.0.to_ne_bytes();
        let y:[u8; 4] = self.1.to_ne_bytes();
        let concat = [x[0],x[1],x[2],x[3],y[0],y[1],y[2],y[3]];
        concat
    }

    // returns all the hex coords that are
    // within radius number of tiles
    pub fn within_radius(&self, radius: i32) -> Vec<HexCoord> {
        let mut within = Vec::new();
        for x in -radius..radius+1 {
            for y in max(-radius, (-x)-radius)..min(radius,(-x)+radius)+1 {
                within.push(HexCoord(x+self.0, y+self.1));
            }
        }
        return within;
    }
}

impl PartialEq for HexCoord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[derive(Component, Inspectable)]
pub struct HexGrid;

 impl HexGrid {
    fn spawn(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        height_map: Res<HeightMap>
    ) {

        let tile_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
        let hex_tile_mesh: Handle<Mesh> = assets.load("meshes/hex.glb#Mesh0/Primitive0");

        let mut tiles = Vec::new();
        for hex_coord in HexCoord(0,0).within_radius(HEX_GRID_RADIUS).into_iter() {
            let tile = HexTile::spawn(hex_coord,height_map.get_height(hex_coord), &mut commands, &hex_tile_mesh, &tile_material);
            tiles.push(tile);
        }
        commands
        .spawn(SpatialBundle{..default()})
        .insert(Name::new("HexGrid"))
        .insert(HexGrid)
        .push_children(&tiles);
    }
 }


#[derive(Component, Inspectable)]
pub struct HexTile;

impl HexTile {
    fn spawn(
        hex_coord: HexCoord,
        height: u32,
        commands: &mut Commands,
        mesh: &Handle<Mesh>,
        material: &Handle<StandardMaterial>
    ) -> Entity {
        let height = height_map::to_world(height);
        let mut position = hex_coord.to_world();
        position.y = height / 2.;
        let scale = Vec3::new(1.,height,1.);
        commands
            .spawn(PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform {
                    translation: position,
                    scale,
                    ..default()
                }
                ,
                ..Default::default()
            })
            .insert(Name::new("HexTile"))
            .insert(HexTile)
            .insert(hex_coord)
            .insert(PickableBundle::default())
            .id()
    }
}