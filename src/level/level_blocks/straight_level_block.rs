use bevy::{color::Color, math::vec2, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use rand::Rng;

use crate::{bevy::car_scene::cm_to_m, level::{level_builder::{LevelBuilder, LevelBuilderContext}, level_builder_operation::LevelBuilderOperation}, v4::{particle::Particle, shape_builder::{line_segment::LineSegment, shape_builder::ShapeBuilder}}};

use super::level_block::LevelBlockComponent;

pub struct StraightLevelBlock {
}

impl LevelBuilderOperation for StraightLevelBlock {
    fn type_name(&self) -> &str {"StraightLevelBlock"}

    fn box_clone(&self) -> Box<dyn LevelBuilderOperation + Send + Sync> {
        Box::new(StraightLevelBlock {})
    }

    fn execute(&self, level_builder_context: &mut LevelBuilderContext) {
        // https://bevyengine.org/examples/2d-rendering/2d-shapes/
        // https://bevyengine.org/examples/3d-rendering/3d-shapes/
        let commands = &mut level_builder_context.commands;
        let meshes = &mut level_builder_context.meshes;
        let materials = &mut level_builder_context.materials;
        //let particle_sim = &mut level_builder_context.particle_sim;

        // Generate a random color
        let mut rng = rand::thread_rng();
        let random_color = Color::rgb(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        );

        let width = rng.gen_range(5.0..=10.0);
        let height = rng.gen_range(-1.5..=1.5);
 
 /* 
        // todo: https://github.com/bevyengine/bevy/discussions/15280
        // draw an AABB for this level block
        let rectangle = Rectangle::new(width, height + 10.0); // Add random height to base height
        commands.spawn((
            LevelBlockComponent {
            }, 
            PbrBundle {
                mesh: meshes.add(rectangle),
                material: materials.add(random_color),
                transform: Transform::from_xyz(
                    level_builder_context.cursor.x - width / 2.0,
                    level_builder_context.cursor.y + height / 2.0,
                    0.0,
                ),
                ..default()
            }
        ));
*/

        let cursor_start = level_builder_context.cursor;
        let cursor_end = cursor_start + vec2(width * level_builder_context.x_direction, height);

        let mut sb = ShapeBuilder::new();
        sb.set_particle_template(level_builder_context.particle_template.clone())
            .apply_operation(LineSegment::new(level_builder_context.cursor, cursor_end)) 
            .create_in_particle_sim(level_builder_context.particle_sim);


        // Update the cursor to the right side of the spawned rectangle
        level_builder_context.cursor = cursor_end;
    }
}
