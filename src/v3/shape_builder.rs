use super::{particle_accelerator::{ParticleAccelerator, ParticleHandle}, types::Vec2};


struct ParticlePrim {
    pos: Vec2,
    radius: f32,
    mass: f32,
    is_static: bool
}

impl ParticlePrim {
    pub fn new(pos: Vec2, radius: f32, mass: f32, is_static: bool) -> Self {
        Self { pos, radius, mass, is_static }
    }
}

pub struct ShapeBuilder {
    particles: Vec<ParticlePrim>,
    is_static: bool,
    mass: f32
}

impl ShapeBuilder {
    pub fn new() -> Self {
        Self { particles: vec![], is_static: false, mass: 1f32 }    
    }

    pub fn set_static(&mut self, is_static: bool) -> &mut Self {
        self.is_static = is_static;
        self
    }

    pub fn set_mass(&mut self, mass: f32) -> &mut Self {
        self.mass = mass;
        self
    }

    pub fn create_particles_in_particle_accelerator(&self, particle_accelerator: &mut ParticleAccelerator, mask: u32) -> Vec<ParticleHandle> {
        let mut handles = vec![];
        for particle in self.particles.iter() {
            let particle_handle = particle_accelerator.create_particle(particle.pos, particle.radius, particle.mass, mask);
            particle_accelerator.set_particle_static(&particle_handle, particle.is_static);
            handles.push(particle_handle);
        }
        handles
    }

    pub fn create_line(&mut self, p1: Vec2, p2: Vec2, radius: f32) -> &mut Self {
        let mut rng = rand::thread_rng();

        let particle_mass = 1.0f32;

        let dist = (p2 - p1).magnitude();
        let divisions = (dist / (radius * 2.0f32)) as usize;
        let delta = (p2 - p1);

        //let col = Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255));
        for i in 0..divisions { 
            let percent = i as f32 / divisions as f32;
            let pos = p1 + (delta * percent);
            self.particles.push(ParticlePrim::new(pos, radius, particle_mass, self.is_static));
        }

        self
    }

}