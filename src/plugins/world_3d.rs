pub mod animate;
pub mod camera;
pub mod config;
pub mod debug;
pub mod hex;
pub mod player;
pub mod sky;

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct World3dPlugins;

impl PluginGroup for World3dPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
        .add(animate::AnimationPlugin)
        .add(camera::CameraPlugin)
        .add(hex::HexPlugin)
        .add(debug::DebugPlugin)
        .add(sky::SkyPlugin)
        .add(player::PlayerPlugin)
    }
}