use super::{particle_accelerator::{ParticleAccelerator, ParticleHandle, StickHandle}, types::Vec2};


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


struct StickPrim {
    particle_indicies: [usize; 2],
    length: f32,
}

impl StickPrim {
    pub fn new(particle_indicies: [usize; 2], particle_positions: [Vec2; 2]) -> Self {
        let length = (particle_positions[1] - particle_positions[0]).magnitude();
        Self { particle_indicies, length }
    }
}

pub struct ShapeBuilder {
    particles: Vec<ParticlePrim>,
    sticks: Vec<StickPrim>,
    is_static: bool,
    mass: f32,

    particle_handles: Vec<ParticleHandle>,
    stick_handles: Vec<StickHandle>,
}


fn convert_to_real_index(idx: i64, len: usize) -> usize {
    if idx >= 0 { idx as usize } else { (len as i64 + idx) as usize }
}

impl ShapeBuilder {
    pub fn new() -> Self {
        Self { particles: vec![], sticks: vec![], is_static: false, mass: 1f32,
            particle_handles: vec![],
            stick_handles: vec![],
        }    
    }

    pub fn set_static(&mut self, is_static: bool) -> &mut Self {
        self.is_static = is_static;
        self
    }

    pub fn set_mass(&mut self, mass: f32) -> &mut Self {
        self.mass = mass;
        self
    }

    pub fn create_in_particle_accelerator(&mut self, particle_accelerator: &mut ParticleAccelerator, mask: u32) -> &mut Self {
        let mut particle_handles = vec![];
        for particle in self.particles.iter() {
            let particle_handle = particle_accelerator.create_particle(particle.pos, particle.radius, particle.mass, mask);
            particle_accelerator.set_particle_static(&particle_handle, particle.is_static);
            particle_handles.push(particle_handle);
        }

        let mut stick_handles = vec![];
        for stick in self.sticks.iter() {
            let stick_handle = particle_accelerator.create_stick([&particle_handles[stick.particle_indicies[0]], &particle_handles[stick.particle_indicies[1]]], stick.length);
            stick_handles.push(stick_handle);
        }

        self.particle_handles = particle_handles;
        self.stick_handles = stick_handles;

        self
    }

    pub fn add_particle(&mut self, pos: Vec2, radius: f32) -> &mut Self {
        self.particles.push(ParticlePrim::new(pos, radius, self.mass, self.is_static));
        self
    }
    

    pub fn add_stick(&mut self, particle_indicies: [i64; 2]) -> &mut Self {
        let real_particle_indicies: [usize; 2] = [
            convert_to_real_index(particle_indicies[0], self.particles.len()),
            convert_to_real_index(particle_indicies[1], self.particles.len()),
        ];
        let particle_positions = [self.particles[real_particle_indicies[0]].pos, self.particles[real_particle_indicies[1]].pos];
        self.sticks.push(StickPrim::new(real_particle_indicies, particle_positions));
        self
    }

    pub fn add_line(&mut self, p1: Vec2, p2: Vec2, radius: f32) -> &mut Self {
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