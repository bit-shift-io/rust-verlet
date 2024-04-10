use std::{cell::RefCell, rc::Rc};

use cgmath::Vector2;

use crate::{sdl_system::SdlSystem};

use super::body::Body;
use super::particle::Particle;


pub struct Solver {
    pub bodies: Vec<Rc<RefCell<Body>>>,
}

impl Solver {
    pub fn new() -> Self {
        Self { bodies: vec![] }
    }

    pub fn add_body(&mut self, body: &Rc<RefCell<Body>>) {
        self.bodies.push(body.clone());
    }

    pub fn update(&mut self, dt: f32) {
        const SUB_STEPS: u32 = 16;
        let sub_dt: f32 = dt / SUB_STEPS as f32;
        for _ in 0..SUB_STEPS {
            self.update_substep(sub_dt);
        }
    }

    pub fn update_substep(&mut self, dt: f32) {
        for body in self.bodies.iter() {
            body.borrow_mut().pre_update(dt);
        }

        self.solve_collisions(dt);

        for body in self.bodies.iter() {
            body.borrow_mut().post_update(dt);
        }
    }

    pub fn solve_collisions(&self, dt: f32) {
        let object_count: &usize = &self.bodies.len();
        for i in 0..*object_count {
            let b1 = &self.bodies[i];

            for k in (&i+1)..*object_count {
                let b2 = &self.bodies[k];    
                self.solve_body_body_collision(b1, b2, dt);
            }

            self.solve_body_self_collision(b1, dt);
        }
    }

    fn compute_movement_weight(&self, a_is_static: bool, b_is_static: bool) -> (f32, f32) {
        // movement weight is used to stop static objects being moved
        let a_movement_weight = if a_is_static { 0.0f32 } else if b_is_static { 1.0f32 } else { 0.5f32 };
        let b_movement_weight = 1.0f32 - a_movement_weight;
        (a_movement_weight, b_movement_weight)
    }

    pub fn solve_body_body_collision(&self, a: &RefCell<Body>, b: &RefCell<Body>, dt: f32) {
        let mut ab = a.borrow_mut();
        let mut bb = b.borrow_mut();
        
        if ab.is_static && bb.is_static {
            return;
        }

        // check if AABB's overlap

        // check between each particle in ab and each particle in bb
        {
            // movement weight is used to stop static objects being moved
            let (a_movement_weight, b_movement_weight) = self.compute_movement_weight(ab.is_static, bb.is_static);

            let a_particles = &ab.particles;
            let b_particles = &bb.particles;

            for a_particle in a_particles.iter() {
                for b_particle in b_particles.iter() {
                    self.solve_particle_particle_collision(a_particle, b_particle, a_movement_weight, b_movement_weight, dt);
                }
            }
        }

        // maybe this would be better called a 'notify_collision' or 'post_collision' handler?
        ab.update_sticks(dt);
        bb.update_sticks(dt);
    }

    pub fn solve_particle_particle_collision(
        &self, 
        a_particle: &Rc<RefCell<Particle>>, 
        b_particle: &Rc<RefCell<Particle>>, 
        a_movement_weight: f32,
        b_movement_weight: f32,
        dt: f32
    ) {
        let collision_axis: Vector2<f32>;
        let dist: f32;
        let min_dist: f32;

        // in a code block so ap and bp borrows are released as we need to borrow mut later if
        // there is a collision
        {
            let ap = a_particle.as_ref().borrow();
            let bp = b_particle.as_ref().borrow();

            collision_axis = ap.pos - bp.pos;
            dist = (collision_axis[0].powf(2f32) + collision_axis[1].powf(2f32)).sqrt();
            min_dist = ap.radius + bp.radius;
        }

        if dist < min_dist as f32 {
            let n: Vector2<f32> = collision_axis / dist;
            let delta: f32 = min_dist as f32 - dist;

            // is it better to have no if statement to make the loop tight at the cost
            // of wasted vector computations?
            let mut ap_mut = a_particle.as_ref().borrow_mut();
            ap_mut.pos += a_movement_weight * delta * n;

            let mut bp_mut = b_particle.as_ref().borrow_mut();
            bp_mut.pos -= b_movement_weight * delta * n;
        }
    }

    pub fn solve_body_self_collision(&self, a: &RefCell<Body>, dt: f32) {
        let ab = a.borrow_mut();
        if !ab.collides_with_self {
            return;
        }

        // movement weight is used to stop static objects being moved
        let (a_movement_weight, b_movement_weight) = self.compute_movement_weight(false, false);

        let a_particles = &ab.particles;

        let object_count: &usize = &a_particles.len();
        for i in 0..*object_count {
            let a_particle = &a_particles[i];
            for k in (&i+1)..*object_count {
                let b_particle = &a_particles[k]; 
                self.solve_particle_particle_collision(a_particle, b_particle, a_movement_weight, b_movement_weight, dt);
            }
        }
    }

    pub fn draw(&self, sdl: &mut SdlSystem) {
        for body in self.bodies.iter() {
            body.borrow().draw(sdl);
        }
    }
}