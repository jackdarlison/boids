use bevy::prelude::*;

use crate::{asset_loader::Assets, moveable::{MoveableObjectBundle, Velocity}};

const NUM_BOIDS: usize = 10;
const BOID_SPEED: f32 = 10.0;

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
        app.add_systems(Startup, spawn_flock);
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
            Boid
        ));
    }
}

