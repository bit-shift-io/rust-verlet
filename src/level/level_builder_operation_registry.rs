use super::{level_blocks::{cliff_operation::CliffOperation, fluid_funnel::FluidFunnel, jelly_cube::JellyCube, saggy_bridge_operation::SaggyBridgeOperation, straight_level_block::StraightLevelBlock}, level_builder_operation::LevelBuilderOperation};


pub struct LevelBuilderOperationRegistry(Vec<Box<dyn LevelBuilderOperation>>);

impl LevelBuilderOperationRegistry {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn register<T: LevelBuilderOperation + 'static>(&mut self, level_builder_operation: T) -> &mut Self {
        self.0.push(Box::new(level_builder_operation));
        self
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clone(&self) -> Self {
        let mut c = LevelBuilderOperationRegistry::new();
        for operation in self.0.iter() {
            c.0.push(operation.box_clone());
        }
        c
    }

    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn LevelBuilderOperation>> {
        self.0.iter()
    }
}