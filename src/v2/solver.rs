use crate::{sdl_system::SdlSystem, v2::body};

use super::body::Body;

pub struct Solver {
    pub bodies: Vec<Box<Body>>,
}

impl Solver {
    pub fn new() -> Self {
        Self { bodies: vec![] }
    }

    pub fn add_body(&mut self, body: Box<Body>) {
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
        for body in self.bodies.iter_mut() {
            body.pre_update(dt);
        }

        // solve collisions
        self.solve_collisions(dt);

        for body in self.bodies.iter_mut() {
            body.post_update(dt);
        }
    }

    pub fn solve_collisions(&mut self, dt: f32) {
        let object_count: &usize = &self.bodies.len();
        for i in 0..*object_count {
            for k in (&i+1)..*object_count {
                let mut b1 = self.bodies[i].as_mut();
                let mut b2 = self.bodies[k].as_mut();

                // do the bounding boxes overlap?
                b1.solve_collision(b2, dt);
            }
        }
    }

    pub fn draw(&self, sdl: &mut SdlSystem) {
        for body in self.bodies.iter() {
            body.draw(sdl);
        }
    }
}