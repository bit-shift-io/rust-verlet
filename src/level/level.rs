use bevy::prelude::*;

use crate::{bevy::car_scene::CarScene, level::level_builder::LevelBuilder};

use super::{level_blocks::{cliff_operation::CliffOperation, drop_direction_reverse::DropDirectionReverse, finish_operation::FinishOperation, fluid_funnel::FluidFunnel, jelly_cube::JellyCube, saggy_bridge_operation::SaggyBridgeOperation, spawn_operation::SpawnOperation, straight_level_block::StraightLevelBlock}, level_builder_operation_registry::LevelBuilderOperationRegistry};

#[derive(Component)]
pub struct LevelComponent {
}

pub fn setup_level(mut commands: Commands, mut query_car_scenes: Query<(&mut CarScene)>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mut car_scene = query_car_scenes.single_mut();
    let particle_sim = &mut car_scene.particle_sim;

    // todo: set random seed based on date
    // https://stackoverflow.com/questions/59020767/how-can-i-input-an-integer-seed-for-producing-random-numbers-using-the-rand-crat

    let mut registry = LevelBuilderOperationRegistry::new();

    // here is our registry
    //
    // things to try:
    // - a jelly draw bridge you drive into and it falls over
    // - a flexible curved pipe that changes direction and flips the car over at the same time
    // - a big ball you drive onto and keep it rolling forwards to get to the other side
    // - an elevator
    // - a steep incline with toothed or flexible ground to give you grip to get up step. (or change the car tyres to be spiked)
    // - some cloth you need to drive under/tear through
    //
    // we should keep a bounding box for each operation applied to help work out if a block can be used instead of using x_direction_changed for example
    registry.register(SpawnOperation {});
    registry.register(FinishOperation {});
    registry.register(StraightLevelBlock {});
    registry.register(SaggyBridgeOperation {});
    registry.register(CliffOperation {});
    registry.register(FluidFunnel {});
    registry.register(JellyCube {});
    registry.register(DropDirectionReverse {});

    let level_builder = LevelBuilder::new(registry).generate(particle_sim, commands, meshes, materials);
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