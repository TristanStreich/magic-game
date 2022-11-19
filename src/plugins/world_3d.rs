pub mod config;
pub mod debug;
pub mod camera;
pub mod hex;

use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::prelude::*;
use bevy_skybox::SkyboxPlugin;
use config::{SUN_HEIGHT, SUN_INTENSITY, SUN_RANGE};

pub struct World3dPlugins;

impl PluginGroup for World3dPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
        .add(camera::CameraPlugin)
        .add(hex::HexPlugin)
        .add(debug::DebugPlugin)
        .add(Sky)
    }
}



pub struct Sky;

impl Plugin for Sky {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ClearColor(Color::rgb(0.5294, 0.8087, 0.9216)))
        .add_startup_system(spawn_sun)
		.add_plugin(SkyboxPlugin::from_image_file("sky.png"));
    }
}

fn spawn_sun(
    mut commands: Commands
) {
    commands
    .spawn(PointLightBundle {
        transform: Transform::from_xyz(0., SUN_HEIGHT, 0.),
        point_light: PointLight {
            intensity: SUN_INTENSITY,
            range: SUN_RANGE,
            ..default()
        },
        ..default()
    });
}