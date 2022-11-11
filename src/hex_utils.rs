use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use std::cmp::{max,min};

pub const HEX_INNER_RADIUS: f32 = 40.0;
pub const HEX_CIRCUMRADIUS: f32 = HEX_INNER_RADIUS * 1.154700538; //sqrt(4/3)
pub const HEX_SMALL_DIAMETER: f32 = 2.0 * HEX_INNER_RADIUS;
pub const HEX_LARGE_DIAMETER: f32 = 2.0 * HEX_CIRCUMRADIUS;

pub const HEX_GRID_RADIUS: i32 = 5;

pub const HEX_SPRITE_SCALE: f32 = HEX_SMALL_DIAMETER * 0.00275;



pub type WorldCoord = (f32, f32);

#[derive(Component, Inspectable, Debug, Copy, Clone)]
pub struct HexCoord(pub i32, pub i32);

impl HexCoord {

    pub fn from_world(world_coord: WorldCoord) -> HexCoord {
        let x = (f32::sqrt(3.0)*world_coord.0 - world_coord.1) / 3.0 / HEX_CIRCUMRADIUS;
        let y = ((2.0/3.0) * world_coord.1) / HEX_CIRCUMRADIUS;
        HexCoord::from_floating((x,y))
    }

    /*
    def axial_round(x, y):
    xgrid = round(x); ygrid = round(y)
    x -= xgrid; y -= ygrid # remainder
    if abs(x) >= abs(y):
        return [xgrid + round(x + 0.5*y), ygrid]
    else:
        return [xgrid, ygrid + round(y + 0.5*x)] */
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

    pub fn to_world(&self) -> WorldCoord {
        let x = HEX_CIRCUMRADIUS * f32::sqrt(3.0) * ((self.0 as f32) + (self.1 as f32) / 2.0);
        let y = HEX_CIRCUMRADIUS * (3.0/2.0) * (self.1 as f32);
        return (x,y);
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
        .add_startup_system_to_stage(StartupStage::PreStartup, HexGrid::spawn);
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