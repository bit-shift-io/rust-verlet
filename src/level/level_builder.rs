use bevy::{math::vec2, prelude::*};
use rand::Rng;

use crate::{bevy::car_scene::cm_to_m, v4::{particle::Particle, particle_sim::ParticleSim}};

use super::level_blocks::{cliff_operation::CliffOperation, finish_operation::FinishOperation, saggy_bridge_operation::SaggyBridgeOperation, spawn_operation::SpawnOperation, straight_level_block::StraightLevelBlock};

pub struct LevelBuilderOperationRegistry {
    level_builder_operations: Vec<Box<dyn LevelBuilderOperation>>,
}

impl LevelBuilderOperationRegistry {
    pub fn new() -> Self {
        let mut result = Self {
            level_builder_operations: vec![],
        };

        // here is our registry
        //result.register_level_builder_operation(SpawnOperation {});
        result.register_level_builder_operation(StraightLevelBlock {});
        result.register_level_builder_operation(SaggyBridgeOperation {});
        result.register_level_builder_operation(CliffOperation {});

        result
    }

    pub fn register_level_builder_operation<T: LevelBuilderOperation + 'static>(&mut self, level_builder_operation: T) -> &mut Self {
        self.level_builder_operations.push(Box::new(level_builder_operation));
        self
    }

    pub fn len(&self) -> usize {
        self.level_builder_operations.len()
    }
}


pub struct LevelBuilder {
    level_builder_operations_registry: Box<LevelBuilderOperationRegistry>,
    cursor: Vec2,
}

impl Default for LevelBuilder {
    fn default() -> Self {
        Self {
            level_builder_operations_registry: Box::new(LevelBuilderOperationRegistry::new()),
            cursor: Vec2::default(),
        }
    }
}

pub struct LevelBuilderContext<'a> {
    pub particle_sim: &'a mut ParticleSim,
    pub cursor: Vec2,
    pub x_direction: f32, // which way the cursor is pointing
    pub commands: Commands<'a, 'a>,
    pub meshes: ResMut<'a, Assets<Mesh>>,
    pub materials: ResMut<'a, Assets<StandardMaterial>>,
    pub particle_template: Particle,
}

pub trait LevelBuilderOperation {
    fn execute(&self, level_builder_context: &mut LevelBuilderContext);
}

impl LevelBuilder {
    
    pub fn generate(&mut self, particle_sim: &mut ParticleSim, mut commands: Commands, meshes: ResMut<Assets<Mesh>>, materials: ResMut<Assets<StandardMaterial>>) -> &mut Self {
        // Algorithm to generate a level
        // 1. Set cursor to origin. This is where the car will spawn (well, a bit behind)
        // 2. Generate a block, which will adjust the cursor
        //self.cursor = vec2(0.0, 0.0);

        let num_blocks = 10;
        let mut rng = rand::thread_rng();

        let particle_radius = cm_to_m(4.0);

        let mut level_builder_context = LevelBuilderContext {
            particle_sim,
            cursor: vec2(0.0, 0.0) ,
            x_direction: -1.0,
            commands,
            meshes,
            materials,
            particle_template: Particle::default().set_static(true).set_color(Color::from(LinearRgba::new(1.0, 1.0, 1.0, 1.0))).set_radius(particle_radius).clone()
        };

        // for now just start with a spawn operation
        let spawn_op = SpawnOperation {};
        spawn_op.execute(&mut level_builder_context);

        for bi in 0..num_blocks {
            // I'm trying to take make a list of operations that can be applied, as well as a number to indicate how
            // likely the operation should be chosen in this stop.
            /*
            let filtered_operations = self.level_builder_operations_registry.level_builder_operations.clone();
            for op in self.level_builder_operations_registry.level_builder_operations.iter() {
                op.as_ref().filter(&mut filtered_operations);
            }*/

            let block_type = rng.gen_range(0..self.level_builder_operations_registry.len());

            let level_builder_operation_box = &self.level_builder_operations_registry.level_builder_operations[block_type];
            let level_builder_operation = level_builder_operation_box.as_ref();

            level_builder_operation.execute(&mut level_builder_context);
        }

        // for now just end with a finish operation
        let finish_op = FinishOperation {};
        finish_op.execute(&mut level_builder_context);

        // let particle system know all static particles have been built - can we move this into create_in_particle_sim?
        level_builder_context.particle_sim.notify_particle_container_changed();

        self
    }

}
