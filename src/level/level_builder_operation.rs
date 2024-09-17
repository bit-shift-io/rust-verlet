use bevy::prelude::*;
use super::level_builder::LevelBuilder;

pub trait LevelBuilderOperation {
    fn execute(&mut self, level_builder: &mut LevelBuilder);
}

pub struct AddStraightLevelBlock {
    
}

impl LevelBuilderOperation for AddStraightLevelBlock {
    fn execute(&mut self, level_builder: &mut LevelBuilder) {
        println!("Adding a straight level block");
    }
}
