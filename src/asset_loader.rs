use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct Assets {
    pub fish: Handle<Scene>,
    pub fish_animation: Handle<AnimationClip>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Assets>()
            .add_systems(PreStartup, load_assets)
            .add_systems(Update, play_animations);
    }
}   

fn load_assets(mut assets: ResMut<Assets>, server: Res<AssetServer>) {
    assets.fish = server.load("Fish.glb#Scene0");
    assets.fish_animation = server.load("Fish.glb#Animation0");
}

fn play_animations(assets: Res<Assets>, mut players: Query<&mut AnimationPlayer>) {
    // Loops over all entities with an AnimationPlayer component and plays the animation
    // Idealy we want the animations to start on different frames
    for mut player in players.iter_mut() {
        player.play(assets.fish_animation.clone()).repeat();
    }
}

