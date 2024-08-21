use bevy::math::Vec2;

use super::particle::{Particle};



pub struct ShapeBuilder {
    pub particles: Vec<Particle>,

    pub particle_template: Particle,
    /* 
    sticks: Vec<StickPrim>,
    springs: Vec<SpringPrim>,

    // particle properties
    is_static: bool,
    mass: f32,
    color: Color,
    radius: f32,
    stiffness_factor: f32,

    // spring properties
    spring_constant: f32,
    elastic_limit: f32,
    damping: f32,

    pub particle_handles: Vec<ParticleHandle>,
    pub stick_handles: Vec<StickHandle>,
    pub spring_handles: Vec<SpringHandle>,
    */
}

impl ShapeBuilder {
    pub fn new() -> Self {
        Self { 
            particles: vec![], 
            particle_template: Particle::default()
        }    
    }

    pub fn set_particle_template(&mut self, particle_template: Particle) -> &mut Self {
        self.particle_template = particle_template;
        self
    }

    pub fn add_particle(&mut self, particle: Particle) -> &mut Self {
        self.particles.push(particle);
        self
    }

    // create a particle from the particle_template and place it at the given position
    // then add it
    pub fn add_particle_at_position(&mut self, pos: Vec2) -> &mut Self {
        let p = self.create_particle().set_position(pos).clone();
        self.add_particle(p);
        self
    }

    // create a particle from the particle_template
    pub fn create_particle(&mut self) -> Particle {
        self.particle_template.clone()
    }

    pub fn add_line(&mut self, p1: Vec2, p2: Vec2) -> &mut Self {
        let dist = (p2 - p1).length();
        let divisions = (dist / (self.particle_template.radius * 2.0)) as usize;
        let delta = p2 - p1;

        for i in 0..divisions { 
            let percent = i as f32 / divisions as f32;
            let pos = p1 + (delta * percent);
            self.add_particle_at_position(pos);
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec2;

    use crate::v4::{particle::{Particle}, shape_builder::ShapeBuilder};

    #[test]
    fn set_particle_properties() {
        let mut b = ShapeBuilder::new();
        b.set_particle_template(Particle::default().set_radius(3.0).clone());
        assert_eq!(b.particle_template.radius, 3.0);
    }

    #[test]
    fn add_particle() {
        let mut b = ShapeBuilder::new();
        b.add_particle(Particle::default().set_position(Vec2::new(1.0, 1.0)).clone());
        assert_eq!(b.particles.len(), 1);
    }
}