use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct Assets {
    pub fish: Handle<Scene>,
    pub fish_animation: Handle<AnimationClip>,
    pub shark: Handle<Scene>,
    pub shark_animation: Handle<AnimationClip>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Assets>()
            .add_systems(PreStartup, load_assets);
            // .add_systems(Update, play_fish_animations);
    }
}   

fn load_assets(mut assets: ResMut<Assets>, server: Res<AssetServer>) {
    assets.fish = server.load("Fish.glb#Scene0");
    assets.fish_animation = server.load("Fish.glb#Animation0");
    assets.shark = server.load("Shark.glb#Scene0");
    assets.shark_animation = server.load("Shark.glb#Animation0");
}

fn play_fish_animations(assets: Res<Assets>, mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>) {
    for (i, mut player) in players.iter_mut().enumerate() {
        player.play(assets.fish_animation.clone_weak()).seek_to((i as f32) * 0.1).repeat();
    }
}

