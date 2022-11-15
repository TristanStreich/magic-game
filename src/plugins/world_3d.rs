pub mod config;
pub mod debug;
pub mod camera;
pub mod hex;

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct World3dPlugins;

impl PluginGroup for World3dPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
        .add(camera::CameraPlugin)
        .add(hex::HexPlugin)
        .add(debug::DebugPlugin)
    }
}
