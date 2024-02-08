use bevy::prelude::*;


pub fn get_top_entity(mut current_entity: Entity, parents: &Query<&Parent>) -> Entity {
    loop {
        if let Ok(parent) = parents.get(current_entity) {
            current_entity = parent.get();
        } else {
            break;
        }
    }
    current_entity
}