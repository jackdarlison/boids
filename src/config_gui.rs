use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::flock::BoidConfig;

pub struct ConfigGuiPlugin;

impl Plugin for ConfigGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup_config_egui);
    }
}

fn setup_config_egui(
    mut contexts: EguiContexts,
    mut boid_config: ResMut<BoidConfig>,
) {
    egui::Window::new("Boid Configuration").show(contexts.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut boid_config.max_speed, 0.0..=100.0).text("Max Speed"));
        ui.add(egui::Slider::new(&mut boid_config.min_speed, 0.0..=100.0).text("Min Speed"));

        ui.add(egui::Slider::new(&mut boid_config.view_angle, 0.0..=std::f32::consts::PI).text("View Angle"));

        ui.add(egui::Slider::new(&mut boid_config.separation_strength, 0.0..=20.0).text("Separation Strength"));
        ui.add(egui::Slider::new(&mut boid_config.separation_range, 0.0..=200.0).text("Separation Range"));
        ui.add(egui::Slider::new(&mut boid_config.alignment_strength, 0.0..=20.0).text("Alignment Strength"));
        ui.add(egui::Slider::new(&mut boid_config.alignment_range, 0.0..=200.0).text("Alignment Range"));
        ui.add(egui::Slider::new(&mut boid_config.cohesion_strength, 0.0..=20.0).text("Cohesion Strength"));
        ui.add(egui::Slider::new(&mut boid_config.cohesion_range, 0.0..=200.0).text("Cohesion Range"));

        ui.add(egui::Slider::new(&mut boid_config.flock_centre_strength, 0.0..=20.0).text("Flock Centre Strength"));
                
        ui.add(egui::Slider::new(&mut boid_config.predator_strength, 0.0..=50.0).text("Predator Strength"));
        ui.add(egui::Slider::new(&mut boid_config.predator_avoidance_strength, 0.0..=50.0).text("Predator Avoidance Strength"));
    });
}