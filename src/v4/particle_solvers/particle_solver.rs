use std::{cell::RefCell, rc::Rc};

use bevy::math::Vec2;

use crate::v4::particle_container::ParticleContainer;



pub trait ParticleSolver {
    fn attach_to_particle_container(&mut self, particle_container: &Rc<RefCell<ParticleContainer>>);
/* 
    fn update_positions(&mut self, particle_accelerator: &mut ParticleContainer, dt: f32);
    */
    fn solve_collisions(&mut self);
    /*
    fn reset_forces(&mut self, particle_accelerator: &mut ParticleContainer, gravity: Vec2);
    */
}