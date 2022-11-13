mod utils;

use bevy::prelude::*;

use utils::camera::CameraPlugin;
use utils::debug::DebugPlugin;
use utils::hex::HexPlugin;
use utils::mouse::MousePlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(CameraPlugin)
    .add_plugin(MousePlugin)
    .add_plugin(HexPlugin)
    .add_plugin(DebugPlugin)
    .run();
}