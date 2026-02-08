use avian2d::PhysicsPlugins;
use bevy::{
    asset::AssetPlugin,
    prelude::{App, DefaultPlugins, PluginGroup},
    window::{Window, WindowMode, WindowPlugin},
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin::default())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::Windowed,
                        title: "curlbox".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(PhysicsPlugins::default())
        .run();
}
