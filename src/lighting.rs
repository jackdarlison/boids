use bevy::prelude::*;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.1, 0.3, 0.6)))
            .insert_resource(AmbientLight {
                brightness: 0.8,
                ..default()
            });
    }
}