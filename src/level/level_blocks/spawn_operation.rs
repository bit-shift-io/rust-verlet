use bevy::{color::Color, math::vec2, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use rand::Rng;

use crate::{bevy::car_scene::cm_to_m, level::{level_builder::{LevelBuilder, LevelBuilderContext}, level_builder_operation::LevelBuilderOperation}, v4::{particle::Particle, shape_builder::{line_segment::LineSegment, shape_builder::ShapeBuilder}}};

pub struct SpawnOperation {
}

impl LevelBuilderOperation for SpawnOperation {
    fn type_name(&self) -> &str {"SpawnOperation"}

    fn box_clone(&self) -> Box<dyn LevelBuilderOperation + Send + Sync> {
        Box::new(SpawnOperation {})
    }

    fn prepare(&self, level_builder_context: &mut LevelBuilderContext, level_builder_operations: &mut Vec<(f32, Box<dyn LevelBuilderOperation + Send + Sync>)>) {
        // ensure that we are always the first operation that gets applied
        // and is never used outside of that range
        if level_builder_context.is_first {
            for op_chance in level_builder_operations.iter_mut() {
                if op_chance.1.type_name() != self.type_name() {
                    op_chance.0 = 0.0;
                }
            }
        } else {
            for op_chance in level_builder_operations.iter_mut() {
                if op_chance.1.type_name() == self.type_name() {
                    op_chance.0 = 0.0;
                }
            }
        }
    }

    fn execute(&self, level_builder_context: &mut LevelBuilderContext) {
        let width = 4.0;
        let height = 0.0;

        let cursor_start = level_builder_context.cursor - vec2(level_builder_context.x_direction * (width * 0.5), 0.0);
        let cursor_end = cursor_start + vec2(width * level_builder_context.x_direction, height);
        
        let mut sb = ShapeBuilder::new();
        sb.set_particle_template(level_builder_context.particle_template.clone())
            .apply_operation(LineSegment::new(cursor_start + vec2(0.0, 1.5), cursor_start))
            .apply_operation(LineSegment::new(cursor_start, cursor_end)) 
            .create_in_particle_sim(level_builder_context.particle_sim);

        level_builder_context.cursor = cursor_end;
    }
}
