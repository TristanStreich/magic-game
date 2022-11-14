// Hex
pub const HEX_INNER_RADIUS: f32 = 0.9;
pub const HEX_CIRCUMRADIUS: f32 = HEX_INNER_RADIUS * 1.154700538; //sqrt(4/3)
pub const HEX_SMALL_DIAMETER: f32 = 2.0 * HEX_INNER_RADIUS;
pub const HEX_LARGE_DIAMETER: f32 = 2.0 * HEX_CIRCUMRADIUS;
pub const HEX_GRID_RADIUS: i32 = 5;


// Camera
pub const CAMERA_SENSITIVITY: f32 = 0.00012;
pub const CAMERA_SPEED: f32 = 12.;
pub const MAX_PITCH: f32 = 0.95;
pub const MIN_PITCH: f32 = 0.25;