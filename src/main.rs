use bevy::prelude::*;

mod lighting;
mod asset_loader;
mod camera;
mod flock;
mod debug;
mod moveable;
mod fps;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(lighting::LightingPlugin)
        .add_plugins(asset_loader::AssetLoaderPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(flock::FlockPlugin)
        .add_plugins(moveable::MoveablePlugin)
        .add_plugins(fps::FpsPlugin)
        // .add_plugins(debug::DebugPlugin)
        .run();
}
