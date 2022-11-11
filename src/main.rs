use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

use magic_game::custom_plugins::flying_camera::{PlayerPlugin, MovementSettings};
use magic_game::custom_plugins::hex_utils::{setup_3d_hex_grid};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup_camera)
    .add_startup_system(setup_3d_hex_grid)
    // .add_plugin(PlayerPlugin)
    // .insert_resource(MovementSettings {
    //     sensitivity: 0.00015, // default: 0.00012
    //     speed: 12.0,          // default: 12.0
    // })

    .run();
}

/// set up a simple 3D scene
fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7.0, 20.0, 4.0),
        )),
        ..Default::default()
    });
}