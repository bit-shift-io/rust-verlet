use super::{particle_accelerator::{ParticleAccelerator, ParticleHandle}, types::Vec2};


struct ParticlePrim {
    pos: Vec2,
    radius: f32,
    mass: f32
}

impl ParticlePrim {
    pub fn new(pos: Vec2, radius: f32, mass: f32) -> Self {
        Self { pos, radius, mass }
    }
}

pub struct ShapeBuilder {
    particles: Vec<ParticlePrim>
}

impl ShapeBuilder {
    pub fn new() -> Self {
        Self { particles: vec![] }    
    }

    pub fn create_particles_in_particle_accelerator(&self, particle_accelerator: &mut ParticleAccelerator, mask: u32) -> Vec<ParticleHandle> {
        let mut handles = vec![];
        for particle in self.particles.iter() {
            handles.push(particle_accelerator.create_particle(particle.pos, particle.radius, particle.mass, mask));
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
            self.particles.push(ParticlePrim::new(pos, radius, particle_mass));
        }

        self
    }

}