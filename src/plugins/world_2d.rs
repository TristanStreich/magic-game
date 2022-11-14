pub mod config;
pub mod camera;
pub mod debug;
pub mod hex;
pub mod mouse;

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub type WorldCoord = (f32, f32);

pub struct World2dPlugins;

impl PluginGroup for World2dPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(camera::CameraPlugin);
        group.add(hex::HexPlugin);
        group.add(mouse::MousePlugin);
        group.add(debug::DebugPlugin);
    }
}