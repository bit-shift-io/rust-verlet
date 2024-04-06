use std::cell::RefCell;

use cgmath::Vector2;

use crate::{sdl_system::SdlSystem, v2::body};

use super::body::Body;


pub struct Solver {
    pub bodies: Vec<RefCell<Body>>,
}

impl Solver {
    pub fn new() -> Self {
        Self { bodies: vec![] }
    }

    pub fn add_body(&mut self, body: RefCell<Body>) {
        self.bodies.push(body);
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
            for k in (&i+1)..*object_count {
                let b2 = &self.bodies[k];
                let b1 = &self.bodies[i];
                self.solve_body_body_collision(b1, b2, dt);
            }
        }
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
            let a_movement_weight = if ab.is_static { 0.0f32 } else if bb.is_static { 1.0f32 } else { 0.5f32 };
            let b_movement_weight = 1.0f32 - a_movement_weight;

            let a_particles = &ab.particles;
            let b_particles = &bb.particles;

            for a_particle in a_particles.iter() {
                for b_particle in b_particles.iter() {

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
                        if !ab.is_static {
                            let mut ap_mut = a_particle.as_ref().borrow_mut();
                            ap_mut.pos += a_movement_weight * delta * n;
                        }
                
                        if !bb.is_static {
                            let mut bp_mut = b_particle.as_ref().borrow_mut();
                            bp_mut.pos -= b_movement_weight * delta * n;
                        }

                        // todo: we only want to update the sticks that are connected to particle i and k
                        //self.update_sticks(dt);
                    }
                }
            }

            /* 
            let object_count: &usize = &self.particles.len();
            for i in 0..*object_count {
                for k in (&i+1)..*object_count {
                    let p1 = self.particles[i].as_ref().borrow();
                    let p2 = self.particles[k].as_ref().borrow();
                    let collision_axis: Vector2<f32> = p1.pos - p2.pos;
                    let dist: f32 = (collision_axis[0].powf(2f32) + collision_axis[1].powf(2f32)).sqrt();
                    let min_dist: f32 = p1.radius + p2.radius;
                    if dist < min_dist as f32 {
                        let n: Vector2<f32> = collision_axis / dist;
                        let delta: f32 = min_dist as f32 - dist;

                        {
                            let mut p1mut = self.particles[i].as_ref().borrow_mut();
                            p1mut.pos += 0.5f32 * delta * n;
                        }
                
                        {
                            let mut p2mut = self.particles[k].as_ref().borrow_mut();
                            p2mut.pos -= 0.5f32 * delta * n;
                        }

                        // todo: we only want to update the sticks that are connected to particle i and k
                        //self.update_sticks(dt);
                    }
                }
            }*/
        }

        // maybe this would be better called a 'notify_collision' or 'post_collision' handler?
        ab.update_sticks(dt);
        bb.update_sticks(dt);
    }

    pub fn draw(&self, sdl: &mut SdlSystem) {
        for body in self.bodies.iter() {
            body.borrow().draw(sdl);
        }
    }
}