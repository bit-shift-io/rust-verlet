use bevy::{color::Color, math::vec2, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use rand::Rng;

use crate::{bevy::car_scene::cm_to_m, level::{level_builder::{LevelBuilder, LevelBuilderContext}, level_builder_operation::LevelBuilderOperation}, v4::{constraints::stick_constraint::StickConstraint, particle::Particle, shape_builder::{line_segment::LineSegment, rectangle, rectangle_stick_grid::RectangleStickGrid, shape_builder::ShapeBuilder}}};

pub struct SaggyBridgeOperation {
}

impl LevelBuilderOperation for SaggyBridgeOperation {
    fn type_name(&self) -> &str {"SaggyBridgeOperation"}

    fn box_clone(&self) -> Box<dyn LevelBuilderOperation + Send + Sync> {
        Box::new(SaggyBridgeOperation {})
    }

    fn execute(&self, level_builder_context: &mut LevelBuilderContext) {
        let rng = &mut level_builder_context.rng;

        let width = rng.gen_range(2.0..=5.0);
        let height = 0.0;

        let rect_height = level_builder_context.particle_template.radius * 4.0;

        let cursor_start = level_builder_context.cursor;
        let cursor_end = cursor_start + vec2(width * level_builder_context.x_direction, height);

        let offset = vec2(0.0, level_builder_context.particle_template.radius * 2.0); // lazy way to fix this!
        let rectangle = rectangle::Rectangle::from_corners(cursor_start + offset, cursor_end + vec2(0.0, -rect_height) + offset);
     
        let mut sb = ShapeBuilder::new();
        sb.set_particle_template(level_builder_context.particle_template.clone().set_static(false).clone());
        sb.apply_operation(RectangleStickGrid::from_rectangle(StickConstraint::default().set_stiffness_factor(500.0).clone(), 
            rectangle));
        
        // set left and right most particles and make them static
        // todo: make this a shape operation?
        let aabb = sb.get_aabb();
        sb.particles.iter_mut().for_each(|particle| {
            if particle.pos.x == aabb.min.x {
                particle.set_static(true);
            }
            if particle.pos.x == aabb.max.x {
                particle.set_static(true);
            }
        });

        sb.create_in_particle_sim(level_builder_context.particle_sim);

        level_builder_context.cursor = cursor_end;
    }
}
