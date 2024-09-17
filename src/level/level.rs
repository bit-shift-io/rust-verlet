use bevy::prelude::*;

use crate::level::level_builder::LevelBuilder;

#[derive(Component)]
struct LevelComponent {
}

pub fn setup_level(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    println!("setup the level");

    let level_builder = LevelBuilder::default().generate(commands, meshes, materials);
}

fn update_level(
    time: Res<Time>, 
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut level_component_query: Query<(&mut LevelComponent)>
) {
    //let mut level_component = level_component_query.single_mut();
    //let delta_seconds = time.delta_seconds();

}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_level)
            .add_systems(Update, update_level);
    }
}