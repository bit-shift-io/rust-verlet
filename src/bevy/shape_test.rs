use bevy::math::Vec2;

use crate::{v3::particle_accelerator::ParticleAccelerator, v4::{particle::Particle, shape_builder::{Circle, ShapeBuilder}}};


// testing out the new v4 shape builder
pub fn shape_test(particle_accelerator: &mut ParticleAccelerator) {
    let mut b = ShapeBuilder::new();
    b.set_particle_template(Particle::default().set_static(true).clone());
    b.add_particles(&Circle::new(Vec2::new(0.0, 0.0), 10.0));
    
    b.create_in_particle_accelerator(particle_accelerator, 0x1);
}