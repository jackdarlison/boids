use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct Assets {
    pub fish: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Assets>()
            .add_systems(PreStartup, load_assets);
    }
}   

fn load_assets(mut assets: ResMut<Assets>, server: Res<AssetServer>) {
    assets.fish = server.load("Fish.glb#Scene0");
}
