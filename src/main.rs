mod hex_utils;
mod debug;

use hex_utils::{HexGridBundle, HexTile, HexCoord};
use debug::{DebugPlugin};

use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(HexGridBundle)
    .add_plugin(DebugPlugin)
    .add_startup_system_to_stage(StartupStage::PreStartup, spawn_camera)
    .add_startup_system(init_highlighted)
    .add_system(highlight_on_click)
    .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}


pub struct Highlighted(Option<HexCoord>);

fn init_highlighted(mut commands: Commands) {
    commands.insert_resource(Highlighted(None));
}

fn highlight_on_click(
    mut commands: Commands,
    mut query: Query<(&HexTile, &HexCoord, &mut Handle<Image>)>,
    assets: Res<AssetServer>,
    mut highlighted: ResMut<Highlighted>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        // Left button was pressed
        for (tile, coord, mut image) in query.into_iter() {
            if coord.0 == 0 && coord.1 == 0 { //TODO: use mouse position for this
                highlighted.0 = match highlighted.0 {
                    Some(_) => None,
                    None => Some(coord.clone())
                };
                println!("Hightlighted = {:?}", highlighted.0);
                //TODO: change sprite of highlighted
                // image = assets.load("hex.png");
            }
        }
    }
}