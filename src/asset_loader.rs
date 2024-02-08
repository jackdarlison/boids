use bevy::{animation, prelude::*, utils::HashMap};

use crate::flock::Boid;

#[derive(Resource, Debug, Default)]
pub struct Assets {
    pub models: HashMap<String, Handle<Scene>>,
    pub animations: HashMap<String, Handle<AnimationClip>>,
}

#[derive(Component)]
pub struct AnimationLink(pub Entity);

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Assets>()
            .add_systems(PreStartup, load_assets)
            .add_systems(Update, (link_animations, init_animations).chain());

    }
}   

fn load_assets(mut assets: ResMut<Assets>, server: Res<AssetServer>) {
    for name in &["Fish", "Shark"] {
        assets.models.insert(name.to_string(), server.load(format!("{}.glb#Scene0", name)));
        assets.animations.insert(name.to_string(), server.load(format!("{}.glb#Animation0", name)));
    }
}

fn get_top_entity(mut current_entity: Entity, parents: &Query<&Parent>) -> Entity {
    loop {
        if let Ok(parent) = parents.get(current_entity) {
            current_entity = parent.get();
        } else {
            break;
        }
    }
    current_entity
}

fn link_animations(mut commands: Commands, animation_players: Query<Entity, Added<AnimationPlayer>>, parents: Query<&Parent>) {
    for ani in animation_players.iter() {
        let top = get_top_entity(ani, &parents);
        commands.entity(top).insert(AnimationLink(ani));
    }
}


fn init_animations(assets: Res<Assets>, mut boids_with_animations: Query<(&Boid, &AnimationLink), Added<AnimationLink>>, mut animation_players: Query<&mut AnimationPlayer>) {
    // Another method may be to derive something from this, removing the need for animation links 
    // let (my_thing, my_thing_entity) = my_thing_query.get_single_mut().expect("Thing not found!");
    // for entity in children.iter_descendants(my_thing_entity) {
    //     if let Ok(mut animation_player) = animation_player.get_mut(entity) {
    //             animation_player.play(animations.0[1].clone_weak()).repeat();
    //     }
    // }
    for (i, (boid, link)) in boids_with_animations.iter_mut().enumerate() {
        let animation = assets.animations.get(&boid.model).expect(format!("No animation for {}", boid.model).as_str()).clone_weak();
        if let Ok(mut player) = animation_players.get_mut(link.0) {
            player.play(animation).seek_to((i as f32) * 0.1).repeat();
        } 
        
    }
}

