use bevy::prelude::*;
use rand::Rng;
use super::level_blocks::{level_block::{LevelBlock, LevelBlockComponent}, straight_level_block::StraightLevelBlock};
use super::level_builder_operation::LevelBuilderOperation;

pub struct LevelBuilder {
    level_blocks: Vec<Box<dyn LevelBlock>>,
    level_block_registry: Vec<Box<dyn LevelBlock>>,
    cursor: Vec2,
    operations: Vec<Box<dyn LevelBuilderOperation>>,
}

impl Default for LevelBuilder {
    fn default() -> Self {
        let mut result = Self {
            level_blocks: vec![],
            level_block_registry: vec![],
            cursor: Vec2::default(),
            operations: vec![],
        };

        // here is our registry
        result.register_level_block(StraightLevelBlock {});

        result
    }
}

impl LevelBuilder {
    pub fn register_level_block<T: LevelBlock + 'static>(&mut self, level_block: T) -> &mut Self {
        self.level_block_registry.push(Box::new(level_block));
        self
    }

    pub fn add_operation(&mut self, operation: Box<dyn LevelBuilderOperation>) -> &mut Self {
        self.operations.push(operation);
        self
    }

    pub fn generate(&mut self) -> &mut Self {
        // Algorithm to generate a level
        // 1. Set cursor to origin. This is where the car will spawn
        // 2. Generate a block, which will adjust the cursor
        self.cursor = Vec2::default();

        let num_blocks = 10;
        let mut rng = rand::thread_rng();
        
        for bi in 0..num_blocks {
            let block_type = rng.gen_range(0..self.level_block_registry.len());

            let level_block_box = &self.level_block_registry[block_type];

            let level_block = level_block_box.as_ref();

            // I probably need the "context" type setup here
            //level_block.apply_to_level_builder(self);

            //level_block.as_ref().apply_to_level_builder(self);
        }

        for operation in self.operations.iter_mut() {
            operation.execute(self);
        }

        self
    }

    pub fn add_to_bevy(&mut self, mut commands: Commands) {
        /* 
        for level_block in self.level_blocks.iter() {
            commands.spawn((
                LevelBlockComponent {
                },
            ));
        }*/

        commands.spawn((
            LevelBlockComponent {
            },
        ));
    }
}
