pub mod config;
pub mod debug;
pub mod camera;
pub mod hex;

use bevy::prelude::App;
use bevy::app::{Plugin, PluginGroup, PluginGroupBuilder};

struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(camera::MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
        });
    }
}

pub struct World3dPlugins;

impl PluginGroup for World3dPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(camera::CameraPlugin)
        group.add(hex::HexPlugin);
        group.add(debug::DebugPlugin);
    }
}
