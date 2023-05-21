use bevy::{prelude::*, winit::WinitSettings};

fn main() {
   App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(WinitSettings::desktop_app())
    .add_startup_system(setup)
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
//Alice, Bob, Carol, Dave, Eve, Mallory, Oscar, and Trent 