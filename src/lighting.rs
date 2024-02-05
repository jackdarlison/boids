use bevy::prelude::*;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.0, 0.6, 1.0)))
            .insert_resource(AmbientLight {
                brightness: 1.0,
                ..default()
            });
    }
}