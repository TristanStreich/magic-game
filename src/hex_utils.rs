use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use std::cmp::{max,min};
use std::collections::HashMap;

pub const HEX_INNER_RADIUS: f32 = 40.0;
pub const HEX_CIRCUMRADIUS: f32 = HEX_INNER_RADIUS * 1.154700538; //sqrt(4/3)
pub const HEX_SMALL_DIAMETER: f32 = 2.0 * HEX_INNER_RADIUS;
pub const HEX_LARGE_DIAMETER: f32 = 2.0 * HEX_CIRCUMRADIUS;

pub const HEX_GRID_RADIUS: i32 = 5;

pub const HEX_SPRITE_SCALE: f32 = HEX_SMALL_DIAMETER * 0.00275;



pub type WorldCoord = (f32, f32);

#[derive(Component, Inspectable, Debug)]
pub struct HexCoord(pub i32, pub i32);

impl HexCoord {

    pub fn from_world(world_coord: WorldCoord) -> HexCoord {
        let x = (f32::sqrt(3.0)*world_coord.0 - world_coord.1) / 3.0 / HEX_CIRCUMRADIUS;
        let y = ((2.0/3.0) * world_coord.1) / HEX_CIRCUMRADIUS;
        return HexCoord(x.round() as i32, y.round() as i32);
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