use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use std::cmp::{max,min};

pub const HEX_INNER_RADIUS: f32 = 40.0;
pub const HEX_CIRCUMRADIUS: f32 = HEX_INNER_RADIUS * 1.154700538; //sqrt(4/3)
pub const HEX_SMALL_DIAMETER: f32 = 2.0 * HEX_INNER_RADIUS;
pub const HEX_LARGE_DIAMETER: f32 = 2.0 * HEX_CIRCUMRADIUS;

pub const HEX_GRID_RADIUS: i32 = 3;

pub const HEX_SPRITE_SCALE: f32 = HEX_SMALL_DIAMETER * 0.00275;


#[derive(Component, Inspectable, Debug)]
pub struct HexCoord(i32,i32);

impl HexCoord {
    pub fn to_world(&self) -> WorldCoord {
        let x = HEX_CIRCUMRADIUS * f32::sqrt(3.0) * ((self.0 as f32) + (self.1 as f32) / 2.0);
        let y = HEX_CIRCUMRADIUS * (3.0/2.0) * (self.1 as f32);
        return (x,y);
    }

    // returns all the hex coords that are
    // within radius number of tiles
    pub fn in_range(&self, radius: i32) -> Vec<HexCoord> {
        let mut within = Vec::new();
        for x in -radius..radius+1 {
            for y in max(-radius, (-x)-radius)..min(radius,(-x)+radius)+1 {
                within.push(HexCoord(x+self.0, y+self.1));
            }
        }
        return within;
    }
}

type WorldCoord = (f32, f32);

// wrong
pub fn world_to_hex(world_coord: WorldCoord) -> HexCoord {
    let x = (f32::sqrt(3.0)*world_coord.0 - world_coord.1) / 3.0 / HEX_CIRCUMRADIUS;
    let y = ((2.0/3.0) * world_coord.1) / HEX_CIRCUMRADIUS;
    return HexCoord(x.round() as i32, y.round() as i32);
}

pub struct HexGrid;

impl Plugin for HexGrid {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(init_grid);
    }
}


fn init_grid(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    for hex_coord in HexCoord(0,0).in_range(HEX_GRID_RADIUS).into_iter() {
        HexTile::spawn_at(hex_coord, &mut commands, &assets)
    }
}


#[derive(Component, Inspectable)]
pub struct HexTile;

impl HexTile {
    fn spawn_at(
        hex_coord: HexCoord,
        commands: &mut Commands,
        assets: &Res<AssetServer>
    ) {
        let (x, y) = hex_coord.to_world();
        commands.spawn_bundle(SpriteBundle {
            texture: assets.load("hex_cropped.png"),
            transform: Transform::from_xyz(x, y, 0.0)
                        .with_scale(Vec3::new(HEX_SPRITE_SCALE, HEX_SPRITE_SCALE, 1.0)),
            ..default()
        })
        .insert(Name::new("HexTile"))
        .insert(hex_coord)
        .insert(HexTile);
    }
}