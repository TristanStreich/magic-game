pub mod config;
pub mod camera;
pub mod debug;
pub mod hex;
pub mod mouse;

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub type WorldCoord = (f32, f32);

pub struct World2dPlugins;

impl PluginGroup for World2dPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
        .add(camera::CameraPlugin)
        .add(hex::HexPlugin)
        .add(mouse::MousePlugin)
        .add(debug::DebugPlugin)
    }
}