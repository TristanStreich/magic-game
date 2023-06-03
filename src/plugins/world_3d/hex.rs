pub mod height_map;

// Standard Lib Imports
use std::cmp::{max,min};

// Bevy Imports
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_mod_picking::{PickableBundle, PickingEvent, SelectionEvent};

use crate::plugins::world_3d::config::{HEX_CIRCUMRADIUS, HEX_GRID_RADIUS};
use height_map::{HeightMap, PerlinGenerator, PerlinStep, RandGenerator, FlatGenerator};

pub struct HexPlugin;

impl Plugin for HexPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PreStartup, init_height_map)
        .add_startup_system(HexGrid::spawn)
        // .add_system(line_drawer)
        ;
    }
}

fn line_drawer( //TODO: remove this. This is for debugging lines
    picked_material: Res<PickedMaterial>,
    mut events: EventReader<PickingEvent>,
    mut tile_query: Query<(&HexCoord, &mut Handle<StandardMaterial>), With<HexTile>>,
) {
    let mut start_line: Option<HexCoord> = None;
    let mut end_line: Option<HexCoord> = None;
    // first check all events
    // looking for tile deselected and tile selected
    for event in events.iter() {
        if let PickingEvent::Selection(selection) = event {
            match selection {
                SelectionEvent::JustSelected(e) => {
                    if let Ok((tile_coord, _)) = tile_query.get(*e) {
                        println!("Selected! {tile_coord:?}");
                        end_line = Some(tile_coord.clone());
                    }
                },
                SelectionEvent::JustDeselected(e) => {
                    if let Ok((tile_coord, _)) = tile_query.get(*e) {
                        println!("Deselected! {tile_coord:?}");
                        start_line = Some(tile_coord.clone());
                    }
                }
            }
        }
    }

    if let (Some(start), Some(end)) = (start_line, end_line) {
        println!("Got Both");
        let line = start.line_between(end);

        println!("Found Line: {line:#?}");
    
        for (coord, mut material) in tile_query.iter_mut() {
            if line.contains(coord) {
                *material = picked_material.0.clone();
            }
        }
    }
}

fn init_height_map(
    mut commands: Commands,
) {
    commands
    // .insert_resource(HeightMap::new(FlatGenerator::new(1)))
    // .insert_resource(HeightMap::new(RandGenerator::new(1, 10, None)))
    // .insert_resource(HeightMap::new(PerlinGenerator::dunes(None)))
    // .insert_resource(HeightMap::new(PerlinGenerator::hills(None)))
    // .insert_resource(HeightMap::new(PerlinGenerator::slopes(None)))
    // .insert_resource(HeightMap::new(PerlinGenerator::crags(None)))
    .insert_resource(HeightMap::new(PerlinGenerator::lowlands(None)))
    // .insert_resource(HeightMap::new(PerlinGenerator::new(vec![
    //     PerlinStep::new(0.05, 0.035, 3.)
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
    pub fn to_world(&self) -> Vec3 {
        let x = HEX_CIRCUMRADIUS * f32::sqrt(3.0) * ((self.0 as f32) + (self.1 as f32) / 2.0);
        let y = HEX_CIRCUMRADIUS * (3.0/2.0) * (self.1 as f32);
        return Vec3 { x: x, y: 0.0, z: y };
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
        let start_world = self.to_world();
        let end_world = other.to_world();
        
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

#[derive(Resource)]
pub struct PickedMaterial(Handle<StandardMaterial>);

 impl HexGrid {
    fn spawn(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        height_map: Res<HeightMap>
    ) {

        let picked_material = materials.add(Color::rgb(0.,0.,0.8).into()); //TODO: remove. This is for debugging lines

        commands.insert_resource(PickedMaterial(picked_material));

        let tile_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
        let hex_tile_mesh: Handle<Mesh> = assets.load("meshes/hex.glb#Mesh0/Primitive0");

        let mut tiles = Vec::new();
        for hex_coord in HexCoord(0,0).within_radius(HEX_GRID_RADIUS).into_iter() {
            let height = height_map.get_height(hex_coord);
            let tile = HexTile::spawn(hex_coord, height, &mut commands, &hex_tile_mesh, &tile_material);
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