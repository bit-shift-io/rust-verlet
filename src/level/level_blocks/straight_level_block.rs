use bevy::{color::Color, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use rand::Rng;

use crate::level::level_builder::{LevelBuilder, LevelBuilderContext, LevelBuilderOperation};

use super::level_block::{LevelBlock, LevelBlockComponent};

pub struct StraightLevelBlock {
}

impl LevelBuilderOperation for StraightLevelBlock {
    fn execute(&self, level_builder_context: &mut LevelBuilderContext) {
        // https://bevyengine.org/examples/2d-rendering/2d-shapes/
        // https://bevyengine.org/examples/3d-rendering/3d-shapes/
        let commands = &mut level_builder_context.commands;
        let meshes = &mut level_builder_context.meshes;
        let materials = &mut level_builder_context.materials;

        // Generate a random color
        let mut rng = rand::thread_rng();
        let random_color = Color::rgb(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        );

        // Generate a random width between 5 and 10
        let random_width = rng.gen_range(5.0..10.0);

        // Generate a random height between -2 and 2
        let random_height = rng.gen_range(-2.0..2.0);

        // create a particle from each particle in the particle_accelerator
        let rectangle = Rectangle::new(random_width, random_height + 10.0); // Add random height to base height
        commands.spawn((
            LevelBlockComponent {
            },
            PbrBundle {
                mesh: meshes.add(rectangle),
                material: materials.add(random_color),
                transform: Transform::from_xyz(
                    level_builder_context.cursor.x,
                    level_builder_context.cursor.y,
                    0.0,
                ),
                ..default()
            }
        ));

        // Update the cursor to the right side of the spawned rectangle
        level_builder_context.cursor.x += random_width;
        level_builder_context.cursor.y += random_height; // Update cursor y
    }
}
