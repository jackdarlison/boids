use std::f32::consts::PI;

use bevy::{prelude::*, utils::HashMap};
use bevy_mod_picking::prelude::*;

use crate::{asset_loader::Assets, moveable::{MoveableObjectBundle, Velocity}, simulation_schedule::InSimulationSchedule};

const NUM_BOIDS: usize = 100;

#[derive(Resource, Debug)]
pub struct BoidConfig {
    min_speed: f32,
    max_speed: f32,
    
    // View angle in radians
    view_angle: f32,

    separation_strength: f32,
    separation_range: f32,
    alignment_strength: f32,
    alignment_range: f32,
    cohesion_strength: f32,
    cohesion_range: f32,

    flock_centre_strength: f32,
}

impl Default for BoidConfig {
    fn default() -> Self {
        Self {
            min_speed: 10.0,
            max_speed: 30.0,
            view_angle: f32::to_radians(80.0),
            separation_strength: 1.0,
            separation_range: 50.0,
            alignment_strength: 1.0,
            alignment_range: 100.0,
            cohesion_strength: 1.0,
            cohesion_range: 200.0,
            flock_centre_strength: 0.2,
        }
    }
}

#[derive(Component, Debug)]
pub struct Flock {
    pub identity: usize,
    pub centre: Vec3,
}

#[derive(Component)]
pub struct Boid;

pub struct FlockPlugin;

impl Plugin for FlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_flock)
            .init_resource::<BoidConfig>()
            .add_systems(Update, (
                apply_boids_rules,
                apply_flock_centre,
            ).in_set(InSimulationSchedule::EntityUpdates));
            
    }
    
}

fn spawn_flock(mut commands: Commands, assets: Res<Assets>, config: Res<BoidConfig>) {
    for _ in 0..NUM_BOIDS {
        let transform = Transform::from_xyz(
            rand::random::<f32>() * 100.0 - 50.0,
            0.0,
            rand::random::<f32>() * 100.0 - 50.0,
        );
        commands.spawn((
            MoveableObjectBundle {
                velocity: Velocity::new(Vec3::new(
                    rand::random::<f32>(),
                    0.0,
                    rand::random::<f32>(),
                ) * config.min_speed),
                model: SceneBundle {
                    scene: assets.fish.clone(),
                    transform,
                    ..default()
                },
            },
            Flock {
                identity: 0,
                centre: Vec3::ZERO,
            },
            Boid,
            PickableBundle::default(),
        ));
    }
}

fn apply_boids_rules(
    mut query: Query<(Entity, &Transform, &mut Velocity), With<Boid>>,
    config: Res<BoidConfig>,
) {
    let mut forces: HashMap<Entity, Vec3> = HashMap::new();

    for (e, t, v) in query.iter() {
        let mut total_separation = Vec3::ZERO;
        let mut total_alignment = Vec3::ZERO;
        let mut total_cohesion = Vec3::ZERO;
        let mut closest_distance = f32::MAX;
        let mut closest_force = Vec3::ZERO;
        for (e2, t2, v2) in query.iter() {
            if e == e2 {continue};
            let angle = v.value.angle_between(t2.translation - t.translation);
            if angle > config.view_angle {continue};

            let distance = t.translation.distance(t2.translation);
            if distance < config.separation_range {
                let separation = (t.translation - t2.translation).normalize_or_zero();
                total_separation += separation;
                if distance < closest_distance{
                    closest_distance = distance;
                    closest_force = separation; 
                }
            }
            if distance < config.alignment_range {
                let alignment = v2.value.normalize_or_zero();
                total_alignment += alignment;
            }
            if distance < config.cohesion_range {
                let cohesion = (t2.translation - t.translation).normalize_or_zero();
                total_cohesion += cohesion;
            }
        }
        let force = total_separation.normalize_or_zero() * config.separation_strength
            + total_alignment.normalize_or_zero() * config.alignment_strength
            + total_cohesion.normalize_or_zero() * config.cohesion_strength
            + closest_force * config.separation_strength;
        forces.insert(e, force);
    }

    for (e, _, mut v) in query.iter_mut() {
        let force = *forces.get(&e).unwrap_or(&Vec3::ZERO);
        v.value = bound_vector(v.value + force, config.min_speed, config.max_speed);
        
    }
}

fn apply_flock_centre(
    mut query: Query<(&Flock, &Transform, &mut Velocity), With<Boid>>,
    config: Res<BoidConfig>,
) {
    for (flock, transform, mut velocity) in query.iter_mut() {
        let force = (flock.centre - transform.translation).normalize_or_zero() * config.flock_centre_strength;
        velocity.value = bound_vector(velocity.value + force, config.min_speed, config.max_speed);
    }
}

fn bound_vector(mut vector: Vec3, min: f32, max: f32) -> Vec3 {
    if vector.length() > max {
        vector = vector.normalize_or_zero() * max;
    } else if vector.length() < min {
        vector = vector.normalize_or_zero() * min;
    }
    vector
}