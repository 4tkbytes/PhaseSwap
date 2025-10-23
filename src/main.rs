use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_systems(Startup, setup)
        // .add_systems(Update, (player_movement, camera_follow).chain())
        .run();
}