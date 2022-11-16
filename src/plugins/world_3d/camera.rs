use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy_inspector_egui::Inspectable;
use bevy_mod_picking::PickingCameraBundle;

use crate::plugins::world_3d::config::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_camera)
        .add_system(orbit_camera)
        .add_system(pan_camera);
    }
}


/// Tags an entity as capable of panning and orbiting.
#[derive(Component, Inspectable)]
struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0
        }
    }
}

/// Spawn a camera like this
fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    commands
    .spawn(Camera3dBundle {
        transform: Transform::from_translation(translation)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()})
    .insert(PanOrbitCamera {
        radius,
        ..Default::default()})
    .insert(Name::new("Game Camera"))
    .insert(PickingCameraBundle::default());
}

// Camera Pan using WASD
fn pan_camera(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PanOrbitCamera)>,
) {
    for (mut transform, mut camera) in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for key in keys.get_pressed() {
            match key {
                KeyCode::W => velocity += forward,
                KeyCode::S => velocity -= forward,
                KeyCode::A => velocity -= right,
                KeyCode::D => velocity += right,
                _ => (),
            }
        }

        velocity = velocity.normalize_or_zero();

        let change = velocity * time.delta_seconds() * CAMERA_SPEED;

        transform.translation += change;
        camera.focus += change;
    }
}


/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
fn orbit_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;

    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }

    for (mut pan_orbit, mut transform) in query.iter_mut() {

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis

            // assert pitch limits
            let mut tilt = (transform.rotation * Vec3::Y).y;
            let below_board = (transform.rotation * Vec3::Z).y < 0.0;
            if below_board {tilt = 2. - tilt;}
            let mut adjustment = 0.0;
            if tilt < MIN_PITCH {
                adjustment = MIN_PITCH - tilt;
            } else if tilt > MAX_PITCH {
                adjustment = MAX_PITCH - tilt;
            } //TODO: max down tilt is a little buggy
            let adjustment = Quat::from_rotation_x(adjustment);
            transform.rotation = transform.rotation * adjustment;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}