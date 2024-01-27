use bevy::prelude::*;

use crate::flock::{Boid, Flock};
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, log_flock);
    }
}

fn log_flock(boids: Query<(Entity, &Transform, &Flock), With<Boid>>) {
    for (e, t, f) in boids.iter() {
        println!("Boid: {:?}, Position: {:?}, Flock: {:?}", e, t.translation, f);
    }
}