use bevy::prelude::*;

mod lighting;
mod asset_loader;
mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(lighting::LightingPlugin)
        .add_plugins(asset_loader::AssetLoaderPlugin)
        .add_plugins(camera::CameraPlugin)
        .run();
}
