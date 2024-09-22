use bevy::{math::vec2, prelude::*};

use crate::{bevy::car_scene::CarScene, level::level_builder::LevelBuilder, random::Random, v4::particle::Particle};

use super::{level_blocks::{cliff_operation::CliffOperation, drop_direction_reverse::DropDirectionReverse, finish_operation::FinishOperation, fluid_funnel::FluidFunnel, jelly_cube::JellyCube, saggy_bridge_operation::SaggyBridgeOperation, spawn_operation::SpawnOperation, straight_level_block::StraightLevelBlock}, level_builder::LevelBuilderContext, level_builder_operation_registry::LevelBuilderOperationRegistry};

use rand::prelude::*;
use rand_seeder::{Seeder, SipHasher};
use rand_pcg::Pcg64;

use chrono::Utc;
use now::DateTimeNow;

#[derive(Component)]
pub struct LevelComponent {
}

pub fn setup_level(mut commands: Commands, mut query_car_scenes: Query<(&mut CarScene)>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mut car_scene = query_car_scenes.single_mut();
    let particle_sim = &mut car_scene.particle_sim;

    // set a random seed used for level generation based on todays date. Each day we get a new map to try
    let mut rng = Random::seed_from_beginning_of_day(); //seed_from_beginning_of_week(); //car_scene.rng;
    
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
    // instead of picking random numbers in a range, pick a random integer and just quantize the number eg. pick a number and then * by 0.5 to get 0.5, 1.0, 1.5, 2.0 as random distances. this might provide more "variety" through less choice.
    // we should keep a bounding box for each operation applied to help work out if a block can be used instead of using x_direction_changed for example
    registry.register(SpawnOperation {});
    registry.register(FinishOperation {});
    registry.register(SaggyBridgeOperation {});
    registry.register(StraightLevelBlock {});
    registry.register(CliffOperation {});
    registry.register(FluidFunnel {});
    registry.register(JellyCube {});
    registry.register(DropDirectionReverse {});

    let mut level_builder_context = LevelBuilderContext::new(particle_sim, &mut rng, commands, meshes, materials);
    let level_builder = LevelBuilder::new(registry).generate(&mut level_builder_context, 20);
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