use sdl2::pixels::Color;

use crate::{point::vec2_to_point, sdl_system::SdlSystem};

use super::particle_accelerator::ParticleAccelerator;


pub struct ParticleRenderer {

}

impl ParticleRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, sdl: &mut SdlSystem, particle_accelerator: &ParticleAccelerator) {
        // draw particles
        for (particle, verlet_position) in particle_accelerator.particles.iter().zip(particle_accelerator.verlet_positions.iter()) {
            if !particle.is_enabled {
                continue;
            }

            sdl.draw_filled_circle(vec2_to_point(verlet_position.pos), particle.radius as i32, particle.color);
            /* 
            let layer_option = particle_accelerator.layer_map.get(&particle.mask);
            layer_option.as_ref().map(|layer| {
                let verlet_position = &layer.verlet_positions[particle.verlet_position_index];
                
            });*/
        }

        // draw sticks
        let col = Color::RGB(0, 128, 0);
        for stick in particle_accelerator.sticks.iter() {
            if !stick.is_enabled {
                continue;
            }
            
            let p1pos = particle_accelerator.verlet_positions[stick.particle_indicies[0]].pos;
            let p2pos = particle_accelerator.verlet_positions[stick.particle_indicies[1]].pos;
            sdl.draw_line(vec2_to_point(p1pos), vec2_to_point(p2pos), col);
        }

        // draw springs
        let col = Color::RGB(0, 0, 128);
        for spring in particle_accelerator.springs.iter() {
            if !spring.is_enabled {
                continue;
            }
            
            let p1pos = particle_accelerator.verlet_positions[spring.particle_indicies[0]].pos;
            let p2pos = particle_accelerator.verlet_positions[spring.particle_indicies[1]].pos;
            sdl.draw_line(vec2_to_point(p1pos), vec2_to_point(p2pos), col);
        }
    }
}
