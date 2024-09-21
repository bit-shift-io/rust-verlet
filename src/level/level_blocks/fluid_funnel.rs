use bevy::{color::Color, math::vec2, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use rand::Rng;

use crate::{bevy::car_scene::{cm_to_m, g_to_kg}, level::{level_builder::{LevelBuilder, LevelBuilderContext}, level_builder_operation::LevelBuilderOperation}, v4::{particle::Particle, shape_builder::{line_segment::LineSegment, rectangle, shape_builder::ShapeBuilder}}};

pub struct FluidFunnel {
}

impl LevelBuilderOperation for FluidFunnel {
    fn type_name(&self) -> &str {"FluidFunnel"}

    fn box_clone(&self) -> Box<dyn LevelBuilderOperation + Send + Sync> {
        Box::new(FluidFunnel {})
    }

    fn default_spawn_chance(&self) -> f32 {
        0.5
    }

    fn prepare(&self, level_builder_context: &mut LevelBuilderContext, level_builder_operations: &mut Vec<(f32, Box<dyn LevelBuilderOperation + Send + Sync>)>) {
        // once the level changes direction, do not spawn a fluid funnel as it takes up to much vertical space and can interfere with stuff above us
        if level_builder_context.x_direction_changed {
            for op_chance in level_builder_operations.iter_mut() {
                if op_chance.1.type_name() == self.type_name() {
                    op_chance.0 = 0.0;
                }
            }
        }
    }

    fn execute(&self, level_builder_context: &mut LevelBuilderContext) {
        let mut rng = rand::thread_rng();

        let width = 0.0;
        let height = rng.gen_range(-2.0..=-0.5);

        let cursor_start = level_builder_context.cursor;
        let cursor_end = cursor_start; //cursor_start + vec2(width * level_builder_context.x_direction, height);


        let particle_radius = level_builder_context.particle_template.radius;
        
        let liquid_particle_radius = particle_radius * 0.85;
        let liquid_particle_mass = g_to_kg(20.0);

        let funnel_height = 3.0;
        
        let funnel_particle_radius = liquid_particle_radius * 0.75;

        let funnel_mouth_half_width = liquid_particle_radius * 6.0 * 0.5;

        let bucket_height = particle_radius * 6.0;
        let bucket_width = 3.0;

        let origin = cursor_start;

        let width = liquid_particle_radius * 2.0 * 20.0;
        let height = liquid_particle_radius * 2.0 * 15.0;

        let mut liquid = ShapeBuilder::new();
        liquid
            .set_particle_template(Particle::default().set_mass(liquid_particle_mass).set_radius(liquid_particle_radius).set_color(Color::from(LinearRgba::BLUE)).clone())
            .apply_operation(rectangle::Rectangle::from_center_size(origin + vec2(0.0, funnel_height + 1.0), vec2(width, height)))
            .create_in_particle_sim(level_builder_context.particle_sim);

        let mut funnel = ShapeBuilder::new();
        funnel
            .set_particle_template(Particle::default().set_static(true).set_radius(funnel_particle_radius).clone())
            .apply_operation(LineSegment::new(origin + vec2(-funnel_mouth_half_width, funnel_height), origin + vec2(-3.0, funnel_height + 2.0))) 
            .apply_operation(LineSegment::new(origin + vec2(funnel_mouth_half_width, funnel_height), origin + vec2(3.0, funnel_height + 2.0))) 
            .create_in_particle_sim(level_builder_context.particle_sim);
 
                /* 
                // bucket
                let mut bucket = ShapeBuilder::new();
                bucket
                    .set_particle_template(Particle::default().set_static(true).set_radius(particle_radius).clone())
                    .apply_operation(LineSegment::new(origin, origin + vec2(bucket_height, -bucket_height))) 
                    .apply_operation(LineSegment::new(origin + vec2(bucket_height, -bucket_height), origin + vec2(bucket_width - bucket_height, -bucket_height)))
                    .apply_operation(LineSegment::new(origin + vec2(bucket_width - bucket_height, -bucket_height), origin + vec2(bucket_width, 0.0)))
                    .create_in_particle_sim(&mut particle_sim);
                */

        level_builder_context.cursor = cursor_end;
    }
}
