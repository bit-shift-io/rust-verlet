use bevy::{color::Color, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

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

        let color = Color::from(LinearRgba::RED);

        // create a particle from each particle in the particle_accelerator
        let rectangle = Rectangle::new(5.0, 10.0);
        commands.spawn((
            LevelBlockComponent {
            },
            PbrBundle {
                mesh: meshes.add(rectangle),
                material: materials.add(color),
                transform: Transform::from_xyz(
                    0.0,
                    0.0,
                    0.0,
                ),
                ..default()
            }
        ));
    }
}
