use bevy::{color::Color, math::vec2, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use rand::Rng;

use crate::{bevy::car_scene::cm_to_m, level::{level_builder::{LevelBuilder, LevelBuilderContext}, level_builder_operation::LevelBuilderOperation}, v4::{constraints::stick_constraint::StickConstraint, particle::Particle, shape_builder::{line_segment::LineSegment, rectangle, rectangle_stick_grid::RectangleStickGrid, shape_builder::ShapeBuilder}}};

pub struct JellyCube {
}

impl LevelBuilderOperation for JellyCube {
    fn type_name(&self) -> &str {"JellyCube"}

    fn box_clone(&self) -> Box<dyn LevelBuilderOperation + Send + Sync> {
        Box::new(JellyCube {})
    }

    fn default_spawn_chance(&self) -> f32 {
        0.5
    }

    fn execute(&self, level_builder_context: &mut LevelBuilderContext) {
        let mut rng = rand::thread_rng();

        let width = 0.0;
        let height = 0.0; //rng.gen_range(-2.0..=-0.5);

        let particle_radius = cm_to_m(4.0);
        let particle_mass = 1.0; //g_to_kg(0.1);

        let origin = level_builder_context.cursor;

        // add a jellow cube to the scene
        ShapeBuilder::new()
            .set_particle_template(Particle::default().set_mass(particle_mass).set_radius(particle_radius).set_color(Color::from(LinearRgba::RED)).clone())
            //.set_constraint_template(StickConstraint::default().set_stiffness_factor(20.0).clone())// this ignores mass
            .apply_operation(RectangleStickGrid::from_rectangle(StickConstraint::default().set_stiffness_factor(20.0).clone(), 
                rectangle::Rectangle::from_center_size(origin + vec2(0.0, 0.5), vec2(0.4, 0.8))))//                 //.add_stick_grid(2, 5, particle_radius * 2.2, Vec2::new(-3.0, cm_to_m(50.0)))
            .create_in_particle_sim(level_builder_context.particle_sim);
    }
}
