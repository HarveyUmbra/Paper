mod layout;
mod components;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(layout::spawn_test)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
//Alice, Bob, Carol, Dave, Eve, Mallory, Oscar, and Trent