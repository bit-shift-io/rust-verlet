use bevy::{color::Color, math::vec2, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use rand::Rng;

use crate::{bevy::car_scene::cm_to_m, level::level_builder::{LevelBuilder, LevelBuilderContext, LevelBuilderOperation}, v4::{particle::Particle, shape_builder::{line_segment::LineSegment, shape_builder::ShapeBuilder}}};

use super::level_block::LevelBlockComponent;

pub struct SpawnOperation {
}

impl LevelBuilderOperation for SpawnOperation {
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

        // Generate a width
        let random_width = 3.0; //rng.gen_range(2.0..3.0);

        // Generate a random height
        let random_height = 0.0;
 
 /* 
        // todo: https://github.com/bevyengine/bevy/discussions/15280
        // draw an AABB for this level block
        let rectangle = Rectangle::new(random_width, random_height + 10.0); // Add random height to base height
        commands.spawn((
            LevelBlockComponent {
            }, 
            PbrBundle {
                mesh: meshes.add(rectangle),
                material: materials.add(random_color),
                transform: Transform::from_xyz(
                    level_builder_context.cursor.x - random_width / 2.0,
                    level_builder_context.cursor.y + random_height / 2.0,
                    0.0,
                ),
                ..default()
            }
        ));
*/

        let cursor = level_builder_context.cursor;
        let cursor_end = cursor + vec2(random_width, random_height);
        // create the ground in the particle system
        let particle_radius = cm_to_m(4.0);
         
        let color = Color::from(LinearRgba::new(1.0, 1.0, 1.0, 1.0));

        let mut sb = ShapeBuilder::new();

        sb.set_particle_template(Particle::default().set_color(color).set_static(true).set_radius(particle_radius * 2.0).clone())
            .apply_operation(LineSegment::new(level_builder_context.cursor + vec2(0.0, 1.5), level_builder_context.cursor))
            .apply_operation(LineSegment::new(level_builder_context.cursor, cursor_end)) 
            .create_in_particle_sim(level_builder_context.particle_sim);


        // let particle system know all static particles have been built - can we move this into create_in_particle_sim?
        level_builder_context.particle_sim.notify_particle_container_changed();

        println!("spawn level block created with {} particles. {} -> {}", sb.particle_handles.len(), level_builder_context.cursor, cursor_end);

        // Update the cursor to the right side of the spawned rectangle
        level_builder_context.cursor = cursor_end;
    }
}
