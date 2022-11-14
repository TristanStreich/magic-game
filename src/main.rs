use bevy::prelude::*;

use magic_game::plugins::world_2d::World2dPlugins;
use magic_game::plugins::world_3d::World3dPlugins;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(World3dPlugins)
    .run();
}

