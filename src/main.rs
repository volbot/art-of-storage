use bevy::prelude::*;

pub mod player;
pub mod world;

use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;

fn main() {
    App::new()
        //.insert_resource(Msaa { samples: 4})          //antialiasing
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldPlugin)
        .run();
}
