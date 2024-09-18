use bevy::prelude::*;

use crate::{bevy::car_scene::CarScene, level::level_builder::LevelBuilder};

#[derive(Component)]
pub struct LevelComponent {
}

pub fn setup_level(mut commands: Commands, mut query_car_scenes: Query<(&mut CarScene)>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mut car_scene = query_car_scenes.single_mut();
    let particle_sim = &mut car_scene.particle_sim;

    let level_builder = LevelBuilder::default().generate(particle_sim, commands, meshes, materials);
}

pub fn update_level(
    time: Res<Time>, 
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut level_component_query: Query<(&mut LevelComponent)>
) {
    //let mut level_component = level_component_query.single_mut();
    //let delta_seconds = time.delta_seconds();

}

/* 
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(Startup, setup_level)
            .add_systems(Update, update_level);
    }
}*/