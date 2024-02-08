use bevy::{pbr::wireframe::WireframePlugin, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;

mod lighting;
mod asset_loader;
mod camera;
mod flock;
mod debug;
mod moveable;
mod fps;
mod simulation_schedule;
mod selected;
mod config_gui;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(WireframePlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(lighting::LightingPlugin)
        .add_plugins(asset_loader::AssetLoaderPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(flock::FlockPlugin)
        .add_plugins(moveable::MoveablePlugin)
        .add_plugins(fps::FpsPlugin)
        .add_plugins(simulation_schedule::SimulationSchedulePlugin)
        .add_plugins(config_gui::ConfigGuiPlugin)
        .add_plugins(selected::SelectedPlugin)
        .add_plugins(debug::DebugPlugin)
        .run();
}
