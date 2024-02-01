use bevy::prelude::*;

#[derive(Component)]
pub struct ConfigGuiRoot;


pub struct ConfigGuiPlugin;

impl Plugin for ConfigGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_config_gui);
    }
}

fn setup_config_gui(
    mut commands: Commands,
) {
    let root_id = commands.spawn((
        ConfigGuiRoot,
        NodeBundle {
            background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
            z_index: ZIndex::Global(i32::MAX),
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Auto,
                top: Val::Percent(1.),
                bottom: Val::Auto,
                left: Val::Percent(1.),
                padding: UiRect::all(Val::Px(4.0)),
                ..Default::default()
            },
            ..Default::default()
        }
    )).id();

    let header_id = commands.spawn((
        TextBundle {
            text: Text::from_section("Boid Configuration", TextStyle::default()),
            ..default()
        },
    )).id();

    commands.entity(root_id).push_children(&[header_id]);

}