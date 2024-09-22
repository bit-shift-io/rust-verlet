use bevy::{math::vec2, prelude::*};
use rand::Rng;
use rand_pcg::Pcg64;

use crate::{bevy::car_scene::cm_to_m, v4::{particle::Particle, particle_sim::ParticleSim}};

use super::{level_blocks::{finish_operation::FinishOperation, spawn_operation::SpawnOperation}, level_builder_operation::LevelBuilderOperation, level_builder_operation_registry::LevelBuilderOperationRegistry};

pub struct LevelBuilder {
    level_builder_operations_registry: LevelBuilderOperationRegistry,
}

impl LevelBuilder {
    pub fn new(level_builder_operations_registry: LevelBuilderOperationRegistry) -> Self {
        Self {
            level_builder_operations_registry,
        }
    }
}

pub struct LevelBuilderContext<'a> {
    pub particle_sim: &'a mut ParticleSim,
    pub cursor: Vec2,
    pub x_direction: f32, // which way the cursor is pointing
    pub x_direction_changed: bool,
    pub commands: Commands<'a, 'a>,
    pub meshes: ResMut<'a, Assets<Mesh>>,
    pub materials: ResMut<'a, Assets<StandardMaterial>>,
    pub particle_template: Particle,
    pub operations: Vec<Box<dyn LevelBuilderOperation + Send + Sync>>,
    pub is_first: bool,
    pub is_last: bool,
    pub rng: &'a mut Pcg64,
}

impl<'a> LevelBuilderContext<'a> {
    pub fn new(particle_sim: &'a mut ParticleSim, rng: &'a mut Pcg64, mut commands: Commands<'a, 'a>, meshes: ResMut<'a, Assets<Mesh>>, materials: ResMut<'a, Assets<StandardMaterial>>) -> Self {
        let particle_radius = cm_to_m(4.0);

        Self {
            particle_sim,
            cursor: vec2(0.0, 0.0),
            x_direction: 1.0,
            x_direction_changed: false,
            commands,
            meshes,
            materials,
            particle_template: Particle::default().set_static(true).set_color(Color::from(LinearRgba::new(1.0, 1.0, 1.0, 1.0))).set_radius(particle_radius).clone(),
            operations: vec![],
            is_first: true,
            is_last: false,
            rng,
        }
    }
}


impl LevelBuilder {
    
    pub fn generate(&mut self, level_builder_context: &mut LevelBuilderContext, num_blocks: i32) -> &mut Self {
        // Algorithm to generate a level
        // 1. Set cursor to origin. This is where the car will spawn (well, a bit behind)
        // 2. Generate a block, which will adjust the cursor

        // currently I spawn an amount of blocks. It might be better to keep spawning blocks till we get a certain distance? or a combination? 
        for bi in 0..num_blocks {
            level_builder_context.is_first = bi == 0;
            level_builder_context.is_last = bi == (num_blocks - 1);

            // 1. Create a pair of "spawn change" and a operation.
            let mut spawn_chance_operations = vec![];
            for op in self.level_builder_operations_registry.iter() {
                spawn_chance_operations.push((op.as_ref().default_spawn_chance(), op.as_ref().box_clone()))
            }

            // 2. Give each operation a chance to mutate "spawn_chance_operations".
            for op in self.level_builder_operations_registry.iter() {
                op.as_ref().prepare(level_builder_context, &mut spawn_chance_operations);
            }

            // 3. Select an operation
            let mut spawn_chance_total = 0.0;
            for (chance, _) in &spawn_chance_operations {
                spawn_chance_total += chance;
            }

            // 4. Find the selected operation and execute it
            let mut spawn_value = level_builder_context.rng.gen_range(0.0..spawn_chance_total);
            for (chance, operation) in &spawn_chance_operations {
                spawn_value -= chance;
                if spawn_value <= 0.0 {
                    // pick this item!
                    level_builder_context.operations.push(operation.box_clone());
                    operation.execute(level_builder_context);
                    break;
                }
            }
        }

        // let particle system know all static particles have been built - can we move this into create_in_particle_sim?
        level_builder_context.particle_sim.notify_particle_container_changed();

        self
    }

}
