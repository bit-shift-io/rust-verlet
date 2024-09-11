use bevy::prelude::*;

use crate::level::level_builder::LevelBuilder;

#[derive(Component)]
pub struct LevelBlockComponent {
}

pub trait LevelBlock {
    fn apply_to_level_builder(&self, level_builder: &mut LevelBuilder);
}