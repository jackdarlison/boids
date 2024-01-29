use bevy::{prelude::*, utils::HashMap};

use crate::{asset_loader::Assets, moveable::{MoveableObjectBundle, Velocity}, simulation_schedule::InSimulationSchedule};

const NUM_BOIDS: usize = 100;
const BOID_SPEED: f32 = 30.0;
const SEPARATION_STRENGTH: f32 = 1.0;
const ALIGNMENT_STRENGTH: f32 = 1.0;
const COHESION_STRENGTH: f32 = 0.9;
const FLOCK_CENTRE_STRENGTH: f32 = 0.2;
const BOID_RANGE: f32 = 50.0;

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
            .add_systems(Update, (
                apply_boids_rules,
                apply_flock_centre,
            ).in_set(InSimulationSchedule::EntityUpdates));
            
    }
    
}

fn spawn_flock(mut commands: Commands, assets: Res<Assets>) {
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
                ) * BOID_SPEED),
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
        ));
    }
}

fn apply_boids_rules(
    mut query: Query<(Entity, &Transform, &mut Velocity), With<Boid>>
) {
    let mut forces: HashMap<Entity, Vec3> = HashMap::new();

    for (e, t, _) in query.iter() {
        let mut force = Vec3::ZERO;
        for (e2, t2, v2) in query.iter() {
            if e == e2 {
                continue;
            }

            let distance = t.translation.distance(t2.translation);
            if distance < BOID_RANGE {
                let separation = (t.translation - t2.translation).normalize();
                let alignment = v2.value.normalize();
                let cohesion = (t2.translation - t.translation).normalize();
                
                force += separation * SEPARATION_STRENGTH;
                force += alignment * ALIGNMENT_STRENGTH;
                force += cohesion * COHESION_STRENGTH;
            }
        }
        forces.insert(e, force);
    }

    for (e, _, mut v) in query.iter_mut() {
        v.value += *forces.get(&e).unwrap_or(&Vec3::ZERO);
        v.value = v.value.normalize() * BOID_SPEED;
    }
}

fn apply_flock_centre(mut query: Query<(&Flock, &Transform, &mut Velocity), With<Boid>>) {
    for (flock, transform, mut velocity) in query.iter_mut() {
        velocity.value += (flock.centre - transform.translation).normalize() * FLOCK_CENTRE_STRENGTH;
        velocity.value = velocity.value.normalize() * BOID_SPEED;
    }
}