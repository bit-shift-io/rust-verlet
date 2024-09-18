use bevy::{math::vec2, prelude::*};
use rand::Rng;

use crate::v4::particle_sim::ParticleSim;

use super::level_blocks::{straight_level_block::StraightLevelBlock};

pub struct LevelBuilderOperationRegistry {
    level_builder_operations: Vec<Box<dyn LevelBuilderOperation>>,
}

impl LevelBuilderOperationRegistry {
    pub fn new() -> Self {
        let mut result = Self {
            level_builder_operations: vec![],
        };

        // here is our registry
        result.register_level_builder_operation(StraightLevelBlock {});

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
    pub commands: Commands<'a, 'a>,
    pub meshes: ResMut<'a, Assets<Mesh>>,
    pub materials: ResMut<'a, Assets<StandardMaterial>>,
}

pub trait LevelBuilderOperation {
    fn execute(&self, level_builder_context: &mut LevelBuilderContext);
}

impl LevelBuilder {
    
    pub fn generate(&mut self, particle_sim: &mut ParticleSim, mut commands: Commands, meshes: ResMut<Assets<Mesh>>, materials: ResMut<Assets<StandardMaterial>>) -> &mut Self {
        // Algorithm to generate a level
        // 1. Set cursor to origin. This is where the car will spawn (well, a bit behind)
        // 2. Generate a block, which will adjust the cursor
        self.cursor = vec2(-1.0, 0.0);

        let num_blocks = 3;
        let mut rng = rand::thread_rng();
        
        let mut level_builder_context = LevelBuilderContext {
            particle_sim,
            cursor: self.cursor.clone(),
            commands,
            meshes,
            materials,
        };

        for bi in 0..num_blocks {
            let block_type = rng.gen_range(0..self.level_builder_operations_registry.len());

            let level_builder_operation_box = &self.level_builder_operations_registry.level_builder_operations[block_type];
            let level_builder_operation = level_builder_operation_box.as_ref();

            level_builder_operation.execute(&mut level_builder_context);
        }

        self
    }

}
