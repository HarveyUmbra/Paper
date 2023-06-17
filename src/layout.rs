use bevy::prelude::*;

use crate::main_menu::main_menu;

pub fn spawn_test(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_test(&mut commands, &asset_server);
}

pub fn build_test(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let test = commands
        .spawn(NodeBundle {
            style: TEST_STYLE,
            background_color: BackgroundColor(Color::rgb(0.15, 0.15, 0.15)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: TEST_STYLE,
                background_color: BackgroundColor(Color::rgb(1.15, 0.15, 0.15)),
                ..Default::default()
            });
            parent.spawn(NodeBundle {
                style: TEST_STYLE,
                background_color: BackgroundColor(Color::rgb(0.15, 1.15, 0.15)),
                ..Default::default()
            });
            parent.spawn(NodeBundle {
                style: TEST_STYLE,
                background_color: BackgroundColor(Color::rgb(0.15, 0.15, 1.15)),
                ..Default::default()
            });
        })
        .id();
    test
}

pub const TEST_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
    ..Style::DEFAULT
};

