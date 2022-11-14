use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;

use magic_game::plugins::world_2d::World2dPlugins;
use magic_game::plugins::world_3d::World3dPlugins;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(DefaultPickingPlugins)
    .add_plugins(World3dPlugins)
    .run();
}

