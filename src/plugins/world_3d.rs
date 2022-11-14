pub mod config;
pub mod debug;
pub mod camera;
pub mod hex;

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct World3dPlugins;

impl PluginGroup for World3dPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
        .add(camera::CameraPlugin)
        .add(hex::HexPlugin)
        .add(debug::DebugPlugin);
    }
}
