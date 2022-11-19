
use bevy::prelude::*;
use crate::plugins::world_3d::config::*;

pub struct SkyPlugin;

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ClearColor(Color::rgb(0.5294, 0.8087, 0.9216)))
        .add_startup_system(spawn_sun);
    }
}

fn spawn_sun(
    mut commands: Commands
) {
    let main_light = commands
    .spawn(DirectionalLightBundle{
        directional_light: DirectionalLight {
            illuminance: SUN_MAIN_INTENSITY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(quat_from_xyz(SUN_MAIN_ROTATION)),
        ..default()
    })
    .insert(Name::new("Main Light"))
    .id();

    let diffuse_light = commands
    .spawn(DirectionalLightBundle{
        directional_light: DirectionalLight {
            illuminance: SUN_DIFFUSE_INTENSITY,
            ..default()
        },
        transform: Transform::from_rotation(quat_from_xyz(SUN_DIFFUSE_ROTATION)),
        ..default()
    })
    .insert(Name::new("Diffuse Light"))
    .id();

    commands.spawn(SpatialBundle::default())
    .insert(Name::new("Sun"))
    .push_children(&[main_light, diffuse_light]);
}


fn quat_from_xyz(rotation: (f32, f32, f32)) -> Quat {
    Quat::from_euler(
        EulerRot::XYZ,
        rotation.0,
        rotation.1,
        rotation.2)
}