use bevy::{pbr::wireframe::Wireframe, prelude::*};
use bevy_mod_picking::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Selected;

pub struct SelectedPlugin;

impl Plugin for SelectedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, highlight_selected);
    }
}

fn highlight_selected(
    mut commands: Commands,
    query: Query<Entity, With<Selected>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for e in query.iter() {
        commands.entity(e).insert((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                ..Default::default()
            },
            // Wireframe,
        ));
    }
}