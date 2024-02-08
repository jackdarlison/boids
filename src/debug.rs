use bevy::prelude::*;

#[derive(Component)]
pub struct EntityLink(pub Entity);

#[derive(Bundle)]
pub struct DebugShape {
    pub linked_to: EntityLink,
    pub pbr: PbrBundle,
}


use crate::flock::{Boid, Flock};
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(PostUpdate, log_flock);
        app.add_systems(Update, (despawn_unlinked_entities, update_debug_shapes).chain());
    }
}

fn log_flock(boids: Query<(Entity, &Transform, &Flock), With<Boid>>) {
    for (e, t, f) in boids.iter() {
        println!("Boid: {:?}, Position: {:?}, Flock: {:?}", e, t.translation, f);
    }
}

fn despawn_unlinked_entities(
    mut commands: Commands,
    debug_entites: Query<(Entity, &EntityLink)>,
) {
    for (e, l) in debug_entites.iter() {
        if commands.get_entity(l.0).is_none() {
            info!("Despawning {:?}", e);
            commands.entity(e).despawn();
        }
    }
}

fn update_debug_shapes(
    mut debug_shapes: Query<(&mut Transform, &EntityLink)>,
    other_transforms: Query<&Transform, Without<EntityLink>>,
) {
    for (mut t, l) in debug_shapes.iter_mut() {
        if let Ok(linked_translation) = other_transforms.get(l.0) {
            t.translation = linked_translation.translation;
        }
    }
}