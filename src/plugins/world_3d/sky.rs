
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
    commands
    .spawn(DirectionalLightBundle{
        directional_light: DirectionalLight {
            illuminance: SUN_INTENSITY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(quat_from_xyz(SUN_ROTATION)),
        ..default()
    })
    .insert(Name::new("Sun"));
    
    commands.insert_resource(AmbientLight {
        brightness: SUN_AMBIENT_LIGHT,
        ..default()
        });
}


fn quat_from_xyz(rotation: (f32, f32, f32)) -> Quat {
    Quat::from_euler(
        EulerRot::XYZ,
        rotation.0,
        rotation.1,
        rotation.2)
}