// Hex
pub const HEX_INNER_RADIUS: f32 = 0.88;
pub const HEX_CIRCUMRADIUS: f32 = HEX_INNER_RADIUS * 1.154700538; //sqrt(4/3)
pub const HEX_SMALL_DIAMETER: f32 = 2.0 * HEX_INNER_RADIUS;
pub const HEX_LARGE_DIAMETER: f32 = 2.0 * HEX_CIRCUMRADIUS;
pub const HEX_GRID_RADIUS: i32 = 50;
pub const HEX_HEIGHT_SCALE: f32 = 0.2;


// Camera
pub const CAMERA_SPEED: f32 = 0.4;
pub const CAMERA_SPEED_OFFSET: f32 = 10.;
pub const MAX_PITCH: f32 = 0.95;
pub const MIN_PITCH: f32 = 0.25;
pub const MAX_ZOOM_IN: f32 = 5.;
pub const MAX_ZOOM_OUT: f32 = 50.;


// Sun
pub const SUN_INTENSITY: f32 = 50_000.;
pub const SUN_ROTATION: (f32, f32, f32) = (11.4,0.3,0.);
pub const SUN_AMBIENT_LIGHT: f32 = 1.;