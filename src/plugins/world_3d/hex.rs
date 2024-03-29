pub mod height_map;

// Standard Lib Imports
use std::cmp::{max,min};

// Bevy Imports
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_mod_picking::{PickableBundle, PickingEvent, SelectionEvent};

use crate::plugins::world_3d::config::{HEX_CIRCUMRADIUS, HEX_GRID_RADIUS};
use height_map::HeightMap;

pub struct HexPlugin;

impl Plugin for HexPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PreStartup, init_height_map)
        .add_startup_system(HexGrid::spawn);
    }
}

fn init_height_map(
    mut commands: Commands,
) {
    commands
    // .insert_resource(HeightMap::new(height_map::FlatGenerator::new(1)))
    // .insert_resource(HeightMap::new(height_map::RandGenerator::new(1, 10, None)))
    // .insert_resource(HeightMap::new(height_map::PerlinGenerator::dunes(None)))
    // .insert_resource(HeightMap::new(height_map::PerlinGenerator::hills(None)))
    // .insert_resource(HeightMap::new(height_map::PerlinGenerator::slopes(None)))
    // .insert_resource(HeightMap::new(height_map::PerlinGenerator::crags(None)))
    .insert_resource(HeightMap::new(height_map::PerlinGenerator::lowlands(None)))
    // .insert_resource(HeightMap::new(height_map::PerlinGenerator::new(vec![
    //     height_map::PerlinStep::new(0.05, 0.035, 3.)
    // ], None)))
    ;
}

#[derive(Component, Inspectable, Debug, Copy, Clone)]
/// Coordinates in axial space
/// see: https://www.redblobgames.com/grids/hexagons/#coordinates-axial
/// HexCoord(q, r)
pub struct HexCoord(pub i32, pub i32);

impl HexCoord {
    /// see: https://www.redblobgames.com/grids/hexagons/#hex-to-pixel-axial
    /// 
    /// Optionally provide height map to get position at the top of the tile.
    /// Otherwise just set height to 0
    pub fn to_world(&self, map: Option<&HeightMap>) -> Vec3 {
        let x = HEX_CIRCUMRADIUS * f32::sqrt(3.0) * ((self.0 as f32) + (self.1 as f32) / 2.0);
        let y = if let Some(map) = map {map.get_world_height(*self)} else {0.};
        let z = HEX_CIRCUMRADIUS * (3.0/2.0) * (self.1 as f32);
        return Vec3 { x, y, z };
    }
    
    /// Uses just x and z componeents of world coord to convert to hexcoord
    /// See: https://www.redblobgames.com/grids/hexagons/#pixel-to-hex
    pub fn from_world(world_coord: Vec3) -> HexCoord {
        // first convert to hex space
        let x = (f32::sqrt(3.0)*world_coord.x - world_coord.z) / 3.0 / HEX_CIRCUMRADIUS;
        let y = ((2.0/3.0) * world_coord.z) / HEX_CIRCUMRADIUS;
        // then round it to the nearest hex coord
        HexCoord::from_floating((x,y))
    }

    /// Round floating point hex space coords to integer hexcoord
    /// see: https://www.redblobgames.com/grids/hexagons/#rounding
    pub fn from_floating((fx, fy): (f32, f32)) -> HexCoord {
        let mut x = fx.round();
        let mut y = fy.round();
        let rem_x = fx - x;
        let rem_y = fy - y;
        if rem_x.abs() >= rem_y.abs() {
            x += (rem_x + 0.5*rem_y).round();
        } else {
            y += (rem_y + 0.5*rem_x).round();
        }
        HexCoord(x as i32, y as i32)
    }

    pub fn to_bytes(self) -> [u8; 8] {
        let x:[u8; 4] = self.0.to_ne_bytes();
        let y:[u8; 4] = self.1.to_ne_bytes();
        let concat = [x[0],x[1],x[2],x[3],y[0],y[1],y[2],y[3]];
        concat
    }

    /// Distance in hex space to other coord.
    /// See: https://www.redblobgames.com/grids/hexagons/#distances-axial
    pub fn distance(&self, other: HexCoord) -> u64 {
        ( (self.0 - other.0).abs()
        + (self.0 + self.1 - other.0 - other.1).abs()
        + (self.1 - other.1).abs()
        ) as u64 / 2
    }

    /// Gets the hexcoords that draw a straight line between self and other
    /// See: https://www.redblobgames.com/grids/hexagons/#line-drawing
    pub fn line_between(&self, other: HexCoord) -> Vec<HexCoord> {
        let start_world = self.to_world(None);
        let end_world = other.to_world(None);
        
        let dist = self.distance(other);
        let mut results = Vec::new();
        for point in 0..=dist {
            let inter_world = start_world.lerp(end_world, (point as f32) / (dist as f32));
            let inter_hex = HexCoord::from_world(inter_world);
            results.push(inter_hex);
        }
        results
    }

    /// returns all the hex coords that are
    /// within radius number of tiles
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
            let tile = HexTile::spawn(hex_coord, &height_map, &mut commands, &hex_tile_mesh, &tile_material);
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
        height_map: &HeightMap,
        commands: &mut Commands,
        mesh: &Handle<Mesh>,
        material: &Handle<StandardMaterial>
    ) -> Entity {
        let height = height_map.get_world_height(hex_coord);
        let mut position = hex_coord.to_world(None);
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