mod hex_utils;
mod debug;

use hex_utils::{HexGridBundle, HexCoord, WorldCoord, HEX_SPRITE_SCALE, HexGrid};
use debug::{DebugPlugin};

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy::render::camera::RenderTarget;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(HexGridBundle)
    .add_plugin(DebugPlugin)
    .add_startup_system_to_stage(StartupStage::PreStartup, spawn_camera)
    .add_startup_system(init_highlighted)
    .add_startup_system(init_mouse_pos)
    .add_system(update_mouse_pos)
    .add_system(highlight_on_click)
    .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}


pub struct Highlighted(Option<HexCoord>);

fn init_highlighted(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<Entity, With<HexGrid>>
) {
    commands.insert_resource(Highlighted(None));
    
    let (x, y) = HexCoord(0,0).to_world();
    let highlighted_hex = commands.spawn_bundle(SpriteBundle {
        texture: assets.load("hex_highlighted.png"),
        transform: Transform::from_xyz(x, y, -1.0)
                    .with_scale(Vec3::new(HEX_SPRITE_SCALE, HEX_SPRITE_SCALE, 1.0)),
        ..default()
    })
    .insert(Name::new("Highlighted Hex"))
    .insert(HighlightedHex)
    .id();

    commands.entity(query.single()).add_child(highlighted_hex);
}

#[derive(Component, Inspectable)]
pub struct HighlightedHex;

fn highlight_on_click(
    mut query: Query<&mut Transform, With<HighlightedHex>>,
    mut highlighted: ResMut<Highlighted>,
    buttons: Res<Input<MouseButton>>,
    mouse_pos: Res<MousePos>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let mut high_transform = query.single_mut();

        let mouse_hex = HexCoord::from_world(mouse_pos.0);
        match &highlighted.0 {
            Some(high_coord) => {
                if *high_coord == mouse_hex {
                    highlighted.0 = None;
                } else {
                    highlighted.0 = Some(mouse_hex);
                }
            },
            None => highlighted.0 = Some(mouse_hex)
        }
        match &highlighted.0 {
            Some(high_coord) => {
                let (x, y) = high_coord.to_world();
                high_transform.translation = Vec3::new(x,y,2.0);
            },
            None => {
                let (x, y) = HexCoord(0,0).to_world();
                high_transform.translation = Vec3::new(x,y,-1.0);
            } 
        }
    }
}

pub struct MousePos(WorldCoord);

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