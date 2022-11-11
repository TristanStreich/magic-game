// Standard Lib Imports
use std::cmp::{max,min};

// Bevy Imports
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

// Custom Modules
use crate::config::hex_grid::{HEX_CIRCUMRADIUS, HEX_GRID_RADIUS, HEX_SPRITE_SCALE};

pub type WorldCoord = (f32, f32);

#[derive(Component, Inspectable, Debug)]
pub struct HexCoord(pub i32, pub i32);

impl HexCoord {

    pub fn from_world(world_coord: WorldCoord) -> HexCoord {
        let x = (f32::sqrt(3.0)*world_coord.0 - world_coord.1) / 3.0 / HEX_CIRCUMRADIUS;
        let y = ((2.0/3.0) * world_coord.1) / HEX_CIRCUMRADIUS;
        return HexCoord(x.round() as i32, y.round() as i32);
    }

    pub fn for_carter(&self) -> Vec3 {
        let x = HEX_CIRCUMRADIUS * f32::sqrt(3.0) * ((self.0 as f32) + (self.1 as f32) / 2.0);
        let y = HEX_CIRCUMRADIUS * (3.0/2.0) * (self.1 as f32);
        return Vec3 { x: x, y: 0.0, z: y };
    }

    pub fn to_world(&self) -> WorldCoord {
        let x = HEX_CIRCUMRADIUS * f32::sqrt(3.0) * ((self.0 as f32) + (self.1 as f32) / 2.0);
        let y = HEX_CIRCUMRADIUS * (3.0/2.0) * (self.1 as f32);
        return (x,y);
    }

    pub fn clone(&self) -> Self {
        HexCoord(self.0,self.1)
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

pub struct HexGridBundle;

impl Plugin for HexGridBundle {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(HexGrid::spawn);
    }
}

#[derive(Component, Inspectable)]
pub struct HexGrid;

 impl HexGrid {
    fn spawn(
        mut commands: Commands,
        assets: Res<AssetServer>
    ) {
        let mut tiles = Vec::new();
        for hex_coord in HexCoord(0,0).within_radius(HEX_GRID_RADIUS).into_iter() {
            let tile = HexTile::spawn_at(hex_coord, &mut commands, &assets);
            tiles.push(tile);
        }
        commands
        .spawn_bundle(SpatialBundle{..default()})
        .insert(Name::new("HexGrid"))
        .insert(HexGrid)
        .push_children(&tiles);
    }
 }
 
#[derive(Component, Inspectable)]
pub struct HexTile;

impl HexTile {
    fn spawn_at(
        hex_coord: HexCoord,
        commands: &mut Commands,
        assets: &Res<AssetServer>
    ) -> Entity {
        let (x, y) = hex_coord.to_world();
        commands.spawn_bundle(SpriteBundle {
            texture: assets.load("hex.png"),
            transform: Transform::from_xyz(x, y, 0.0)
            .with_scale(Vec3::new(HEX_SPRITE_SCALE, HEX_SPRITE_SCALE, 1.0)),
            ..default()
        })
        .insert(Name::new("HexTile"))
        .insert(hex_coord)
        .insert(HexTile)
        .id()
    }
}

pub fn setup_3d_hex_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let hex_3d: Handle<Mesh> = asset_server.load("hex.glb#Mesh0/Primitive0");

    for hex_coord in HexCoord(0,0).within_radius(HEX_GRID_RADIUS).into_iter() {
        let position = hex_coord.for_carter();

        commands
            .spawn_bundle(PbrBundle {
                transform: Transform::from_translation(position),
                ..Default::default()
            })
            // Add children to the parent
            .with_children(|parent| {
                parent.spawn_bundle(PbrBundle {
                    mesh: hex_3d.clone(),
                    material: white_material.clone(),
                    // transform: {
                    //     let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    //     transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    //     transform
                    // },
                    ..Default::default()
                });
            });
    }
}
