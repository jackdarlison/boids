use bevy::{pbr::wireframe::Wireframe, prelude::*};
use bevy_mod_picking::{events::{Click, Pointer}, prelude::ListenerInput};

use crate::{debug::{DebugShape, EntityLink}, utils::get_top_entity};


#[derive(Event, Debug, Clone)]
pub struct SelectedEvent(Entity);

impl From<ListenerInput<Pointer<Click>>> for SelectedEvent {
    fn from(input: ListenerInput<Pointer<Click>>) -> Self {
        SelectedEvent(input.target)
    }
}

pub struct SelectedPlugin;

impl Plugin for SelectedPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectedEvent>()
            .add_systems(Update, handle_selected_event);
    }
}

fn handle_selected_event(
    mut commands: Commands,
    mut selected_events: EventReader<SelectedEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    parents: Query<&Parent>,
) {
    for event in selected_events.read() {
        // pointer event seems to get return some entity used for detection, not the actual entity
        // Get the top entity in the hierarchy which has the correct tranform to follow
        let top = get_top_entity(event.0, &parents);
        commands.spawn((
            DebugShape {
                linked_to: EntityLink(top),
                pbr: PbrBundle {
                    mesh: meshes.add(Mesh::try_from(shape::Icosphere { radius: 100.0, ..Default::default()}).unwrap()),
                    material: materials.add(Color::rgba(1.0, 1.0, 1.0, 0.05).into()),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..Default::default()
                },
            },
            Wireframe,
        ));
    }
}