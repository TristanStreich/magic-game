use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

use crate::plugins::world_2d::WorldCoord;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(init_mouse_pos)
        .add_system(update_mouse_pos);
    }
}

#[derive(Resource)]
pub struct MousePos(WorldCoord);

impl MousePos {
    pub fn get_world_coords(&self) -> WorldCoord {
        return self.0;
    }
}

fn init_mouse_pos(
    mut commands: Commands,
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>
) {
    let mouse_pos = get_mouse_pos(wnds, q_camera);
    commands.insert_resource(MousePos(mouse_pos));
}

fn update_mouse_pos(
    mut cur_mouse_pos: ResMut<MousePos>,
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>
) {
    let new_mouse_pos = get_mouse_pos(wnds, q_camera);
    cur_mouse_pos.0 = new_mouse_pos;
}

// copied this from bevy cheat sheet lets see if it works
fn get_mouse_pos(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>
) -> WorldCoord {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        (world_pos.x, world_pos.y)
    } else {
        (0.0,0.0) //TODO: replace this. Maybe an option
    }
}