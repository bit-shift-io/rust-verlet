use std::collections::HashMap;

use sdl2::pixels::Color;

use crate::{point::vec2_to_point, sdl_system::SdlSystem};

use super::types::Vec2;


pub struct ParticleHandle {
    id: usize,
}

impl ParticleHandle {
    fn new(id: usize) -> Self {
        Self { id }
    }
}

struct VerletPosition {
    pos: Vec2,
    pos_prev: Vec2,
}

struct Layer {
    mask: u32,

    // todo: split into dynamic and static particle positions?
    verlet_positions: Vec<VerletPosition>,
}

impl Layer {
    fn new(mask: u32) -> Self {
        Self {
            mask,
            verlet_positions: vec![],
        }
    }
}

struct Particle {
    id: usize,
    radius: f32,
    mass: f32,
    mask: u32,
    verlet_position_index: usize,
}

pub struct ParticleAccelerator {
    particles: Vec<Particle>,
    layer_map: HashMap<u32, Layer>,
}

impl ParticleAccelerator {
    pub fn new() -> Self {
        Self {
            particles: vec![],
            layer_map: HashMap::new(),
        }
    }

    /* 
    fn create_layer(&mut self, mask: u32) {
        let layer = Layer::new(mask);
        self.layer_map.insert(mask, layer);
    }*/

    pub fn create_particle(&mut self, pos: Vec2, radius: f32, mass: f32, mask: u32) -> ParticleHandle {
        let layer = self.layer_map.entry(mask).or_insert(Layer::new(mask));
        let verlet_position_index = layer.verlet_positions.len();

        let id = self.particles.len();
        let particle = Particle {
            id,
            radius,
            mass,
            mask,
            verlet_position_index
        };
        self.particles.push(particle);

        layer.verlet_positions.push(VerletPosition { pos, pos_prev: pos });
        
        ParticleHandle::new(id) 
    }
}

pub struct ParticleCollider {

}

impl ParticleCollider {
    pub fn new() -> Self {
        Self {}
    }

    pub fn solve_collisions(&mut self, particle_accelerator: &mut ParticleAccelerator) {
        println!("todo: ParticleCollider::solve_collisions");
    }
}

pub struct ParticleRenderer {

}

impl ParticleRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, sdl: &mut SdlSystem, particle_accelerator: &ParticleAccelerator) {
        let col = Color::RGB(128, 0, 0);

        for particle in particle_accelerator.particles.iter() {
            let layer_option = particle_accelerator.layer_map.get(&particle.mask);
            layer_option.as_ref().map(|layer| {
                let verlet_position = &layer.verlet_positions[particle.verlet_position_index];
                sdl.draw_filled_circle(vec2_to_point(verlet_position.pos), particle.radius as i32, col);
            });
        }
    }
}