use sdl2::pixels::Color;

use crate::v3::{particle_accelerator::{ParticleAccelerator, ParticleHandle, StickHandle}, shape_builder::ShapeBuilder, types::Vec2};

use super::car_scene::CarSceneContext;

pub struct Cloth {
    particle_handles: Vec<ParticleHandle>,
    stick_handles: Vec<StickHandle>,
}

impl Cloth {
    pub fn new(particle_accelerator: &mut ParticleAccelerator) -> Self {
        let mask = 0x0; // disable collisions
        let mut builder = ShapeBuilder::new();
        builder.add_cloth_grid(10, 10, 20.0, Vec2::new(100.0, 100.0))
            .create_in_particle_accelerator(particle_accelerator, mask);

        Self {
            particle_handles: builder.particle_handles,
            stick_handles: builder.stick_handles
        }
    }

    pub fn update(&mut self, car_scene_context: &mut CarSceneContext) {
        let drag = 0.01f32;
        let elasticity = 10.0f32;
        let tear_distance_percent = 3.0f32;

        // compute distance from point to the mouse cursor
        // if in range, change the colour
        for particle_handle in self.particle_handles.iter() {
            let pos = car_scene_context.particle_accelerator.get_particle_position(particle_handle);
            let mouse_dir = pos - car_scene_context.mouse.pos;
            let mouse_dist = mouse_dir.magnitude();
            let is_selected = mouse_dist < car_scene_context.mouse.cursor_size;
            let color = if is_selected { Color::RGB(128, 0, 0) } else { Color::RGB(0, 128, 0) };
            car_scene_context.particle_accelerator.set_particle_color(particle_handle, color);
        }

        // if left mouse down, drag particles
        if car_scene_context.mouse.left_button_down {
            let mut difference = car_scene_context.mouse.pos - car_scene_context.mouse.pos_prev;

            if difference.x > elasticity {
                difference.x = elasticity
            }
            if difference.y > elasticity {
                difference.y = elasticity
            }
            if difference.x < -elasticity {
                difference.x = -elasticity
            }
            if difference.y < -elasticity {
                difference.y = -elasticity
            }
            
            for particle_handle in self.particle_handles.iter() {
                // push the particles previous position by the difference
                let pos = car_scene_context.particle_accelerator.get_particle_position(particle_handle);
                let pos_prev = pos - difference;
                car_scene_context.particle_accelerator.set_particle_position_previous(particle_handle, pos_prev);
            }
        }

        // if right mouse down, cut sticks
        if car_scene_context.mouse.left_button_down {
        }
    }
}