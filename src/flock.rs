use std::cmp::max;

use bevy::{prelude::*, utils::HashMap};
use bevy_mod_picking::prelude::*;

use crate::{asset_loader::Assets, moveable::{MoveableObjectBundle, Velocity}, simulation_schedule::InSimulationSchedule};

const NUM_BOIDS: usize = 100;

#[derive(Resource, Debug)]
pub struct BoidConfig {
    pub min_speed: f32,
    pub max_speed: f32,
    
    // View angle in radians
    pub view_angle: f32,

    pub separation_strength: f32,
    pub separation_range: f32,
    pub alignment_strength: f32,
    pub alignment_range: f32,
    pub cohesion_strength: f32,
    pub cohesion_range: f32,

    pub flock_centre_strength: f32,
}

impl Default for BoidConfig {
    fn default() -> Self {
        Self {
            min_speed: 10.0,
            max_speed: 30.0,
            view_angle: f32::to_radians(120.0),
            separation_strength: 5.0,
            separation_range: 50.0,
            alignment_strength: 5.0,
            alignment_range: 75.0,
            cohesion_strength: 5.0,
            cohesion_range: 100.0,
            flock_centre_strength: 2.0,
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

#[derive(Resource)]
pub struct BoidMap {
    pub map: HashMap<(isize, isize, isize), Vec<Entity>>,
    pub resolution: usize,
}

impl Default for BoidMap {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
            resolution: 0,
        }
    }
}

impl BoidMap {
    pub fn reset(&mut self) {
        self.map.clear();
    }

    pub fn add_boid(&mut self, boid: Entity, position: Vec3) {
        let (x, y, z) = self.vec3_to_grid(position);
        let boids = self.map.entry((x, y, z)).or_insert_with(Vec::new);
        boids.push(boid);
    }

    pub fn vec3_to_grid(&self, position: Vec3) -> (isize, isize, isize) {
        // floor the position based on the resolution
        let conversion = |x: f32| -> isize {
            if x < 0.0 {
                // subtract 1 to make sure the conversion is always towards negative
                (x as isize / self.resolution as isize) - 1
            } else {
                x as isize / self.resolution as isize
            }
        };
        (
            conversion(position.x),
            conversion(position.y),
            conversion(position.z),
        )
    }

    pub fn get_possible_neighbours(&self, position: Vec3) -> Vec<Entity> {
        let (x, y, z) = self.vec3_to_grid(position);
        let mut boids = Vec::new();
        // Iterate over the 3x3x3 grid around the boid to collect all possible neighbours
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    boids.extend(
                        self.map.get(&(x + i, y + j, z + k)).unwrap_or(&Vec::new())
                    );
                }
            }
        }
        boids
    }
}

pub struct FlockPlugin;

impl Plugin for FlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_flock)
            .init_resource::<BoidConfig>()
            .init_resource::<BoidMap>()
            .add_systems(Update, (
                update_boid_map,
                (apply_boids_rules, apply_flock_centre),
            ).chain().in_set(InSimulationSchedule::EntityUpdates));
            
    }
    
}

fn spawn_flock(mut commands: Commands, assets: Res<Assets>, config: Res<BoidConfig>) {
    //space boids out depending on the number of boids
    let spatial_separation = 100.0 * (NUM_BOIDS as f32).sqrt();
    for _ in 0..NUM_BOIDS {
        let transform = Transform::from_xyz(
            rand::random::<f32>() * spatial_separation - spatial_separation / 2.0,
            0.0,
            rand::random::<f32>() * spatial_separation - spatial_separation / 2.0,
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

fn update_boid_map(
    mut flocks: ResMut<BoidMap>,
    config: Res<BoidConfig>,
    query: Query<(Entity, &Transform), With<Boid>>,
) {
    flocks.reset();
    flocks.resolution = max(max(
        config.separation_range as usize,
        config.alignment_range as usize,
    ), config.cohesion_range as usize) * 2;
    for (e, t) in query.iter() {
        flocks.add_boid( e, t.translation);
    }
}

fn apply_boids_rules(
    mut query: Query<(Entity, &Transform, &mut Velocity, &Flock), With<Boid>>,
    config: Res<BoidConfig>,
    time: Res<Time>,
    flocks: Res<BoidMap>,
) {
    let mut forces: HashMap<Entity, Vec3> = HashMap::new();

    for (entity1, transform1, velocity1, flock1) in query.iter() {
        let mut total_separation = Vec3::ZERO;
        let mut total_alignment = Vec3::ZERO;
        let mut total_cohesion = Vec3::ZERO;
        let mut closest_distance = f32::MAX;
        let mut closest_force = Vec3::ZERO;
        for entity2 in flocks.get_possible_neighbours(transform1.translation) {
            // ignore self
            if entity1 == entity2 {continue};
            
            // retrieve the components of the other boid
            let (_, transform2, velocity2, flock2) = query.get(entity2).unwrap();

            // check if the other boid is within the view angle
            let angle = velocity1.value.angle_between(transform2.translation - transform1.translation);
            if angle > config.view_angle {continue};

            let distance = transform1.translation.distance(transform2.translation);
            if distance < config.separation_range {
                // values are normalised so that all boids have the same influence
                let separation = (transform1.translation - transform2.translation).normalize_or_zero();
                total_separation += separation;
                if distance < closest_distance{
                    closest_distance = distance;
                    closest_force = separation; 
                }
            }

            // If not in the same flock, ignore alignment and cohesion
            if flock1.identity != flock2.identity {continue};

            if distance < config.alignment_range {
                let alignment = velocity2.value.normalize_or_zero();
                total_alignment += alignment;
            }
            if distance < config.cohesion_range {
                let cohesion = (transform2.translation - transform1.translation).normalize_or_zero();
                total_cohesion += cohesion;
            }
        }
        // values are nomalised so that all forces have the same base influence, regardless of amount of boids in each forces range
        let force = total_separation.normalize_or_zero() * config.separation_strength
            + total_alignment.normalize_or_zero() * config.alignment_strength
            + total_cohesion.normalize_or_zero() * config.cohesion_strength
            + closest_force * config.separation_strength;
        forces.insert(entity1, force);
    }

    for (e, _, mut v, _) in query.iter_mut() {
        let force = *forces.get(&e).unwrap_or(&Vec3::ZERO);
        v.value = bound_vector(v.value + force * time.delta_seconds(), config.min_speed, config.max_speed);
    }
}

fn apply_flock_centre(
    mut query: Query<(&Flock, &Transform, &mut Velocity), With<Boid>>,
    config: Res<BoidConfig>,
    time: Res<Time>,
) {
    for (flock, transform, mut velocity) in query.iter_mut() {
        let force = (flock.centre - transform.translation).normalize_or_zero() * config.flock_centre_strength;
        velocity.value = bound_vector(velocity.value + force * time.delta_seconds(), config.min_speed, config.max_speed);
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

