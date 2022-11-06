use bevy::prelude::*;

pub const HEX_INNER_RADIUS: f64 = 1.0;
pub const HEX_UNIT: f64 = 2.0 * HEX_INNER_RADIUS;
#[derive(Component)]
pub struct HexCoord(i64, i64);

impl HexCoord {
    pub fn to_world(&self) -> WorldCoord {
        let x = HEX_UNIT * ((self.0 as f64) - (self.1 as f64) / 2.0);
        let y = HEX_UNIT * (f64::sqrt(3.0)/2.0) * (self.1 as f64);
        return (x,y);
    }
}

type WorldCoord = (f64, f64);

pub fn world_to_hex(world_coord: WorldCoord) -> HexCoord {
    let x = (world_coord.0 + (world_coord.1) / f64::sqrt(3.0)) / HEX_UNIT;
    let y = ((2.0/f64::sqrt(3.0)) * world_coord.1) / HEX_UNIT;
    return HexCoord(x.round() as i64, y.round() as i64);
}