use bevy::{color::Color, math::vec2, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use rand::Rng;

use crate::{bevy::car_scene::cm_to_m, level::level_builder::{LevelBuilder, LevelBuilderContext, LevelBuilderOperation}, v4::{particle::Particle, shape_builder::{line_segment::LineSegment, shape_builder::ShapeBuilder}}};

pub struct FinishOperation {
}

impl LevelBuilderOperation for FinishOperation {
    fn execute(&self, level_builder_context: &mut LevelBuilderContext) {
        let width = 3.0;
        let height = 0.0;

        let cursor_start = level_builder_context.cursor;
        let cursor_end = cursor_start + vec2(width * level_builder_context.x_direction, height);

        let mut sb = ShapeBuilder::new();
        sb.set_particle_template(level_builder_context.particle_template)
            .apply_operation(LineSegment::new(level_builder_context.cursor, cursor_end)) 
            .apply_operation(LineSegment::new(cursor_end, cursor_end + vec2(0.0, 1.5)))
            .create_in_particle_sim(level_builder_context.particle_sim);

        level_builder_context.cursor = cursor_end;
    }
}
