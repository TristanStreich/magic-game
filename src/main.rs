mod hex_utils;
mod debug;

use hex_utils::HexGridBundle;
use debug::{DebugPlugin};

use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(HexGridBundle)
    .add_plugin(DebugPlugin)
    .add_startup_system_to_stage(StartupStage::PreStartup, spawn_camera)
    .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}