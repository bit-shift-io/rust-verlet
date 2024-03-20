use crate::{sdl_system::SdlSystem, v2::body};

use super::body::Body;

pub struct Solver<'a> {
    pub bodies: Vec<Box<Body<'a>>>,
}

impl<'a> Solver<'a> {
    pub fn new() -> Self {
        Self { bodies: vec![] }
    }

    pub fn add_body(&mut self, body: Box<Body<'a>>) {
        self.bodies.push(body);
    }

    pub fn update(&mut self, dt: f32) {
        const SUB_STEPS: u32 = 16;
        let sub_dt: f32 = dt / SUB_STEPS as f32;
        for _ in 0..SUB_STEPS {
            for body in self.bodies.iter_mut() {
                body.update(sub_dt);
            }
        }
    }

    pub fn draw(&self, sdl: &mut SdlSystem) {
        for body in self.bodies.iter() {
            body.draw(sdl);
        }
    }
}