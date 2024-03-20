use cgmath::InnerSpace;
use sdl2::{pixels::Color, rect::Point};

use crate::sdl_system::SdlSystem;

use super::particle::Particle;

pub struct Stick<'a> {
    pub length: f32,
    pub p1: &'a Box<Particle>,
    pub p2: &'a Box<Particle>,
    //pub color: Color,
}

impl<'a> Stick<'a> {
    pub fn new(p1: &'a Box<Particle>, p2: &'a Box<Particle>) -> Self {
        let length = (p1.pos - p2.pos).magnitude();
        Self { length, p1, p2 }
    }

    pub fn draw(&self, sdl: &mut SdlSystem) {
        sdl.canvas.set_draw_color(Color::RGB(255, 255, 255));

        let p1_x = self.p1.pos[0].round() as i32;
        let p1_y = self.p1.pos[1].round() as i32;

        let p2_x = self.p2.pos[0].round() as i32;
        let p2_y = self.p2.pos[1].round() as i32;

        let _ = sdl.canvas.draw_line(Point::new(p1_x, p1_y), Point::new(p2_x, p2_y));
    }
}