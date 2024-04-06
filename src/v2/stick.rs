use std::{cell::RefCell, rc::Rc};

use cgmath::InnerSpace;
use sdl2::{pixels::Color, rect::Point};

use crate::sdl_system::SdlSystem;

use super::particle::Particle;

pub struct Stick {
    pub length: f32,
    pub p1: Rc<RefCell<Particle>>,
    pub p2: Rc<RefCell<Particle>>,
    //pub color: Color,
}

impl Stick {
    pub fn new(p1: Rc<RefCell<Particle>>, p2: Rc<RefCell<Particle>>) -> Self {
        let pos1 = p1.as_ref().borrow().pos;
        let pos2 = p2.as_ref().borrow().pos;
        let length = (pos1 - pos2).magnitude();
        Self { length, p1, p2 }
    }

    pub fn draw(&self, sdl: &mut SdlSystem) {
        let p1_pos1 = self.p1.as_ref().borrow().pos[0];
        let p1_pos2 = self.p1.as_ref().borrow().pos[1];
        
        let p2_pos1 = self.p2.as_ref().borrow().pos[0];
        let p2_pos2 = self.p2.as_ref().borrow().pos[1];

        sdl.canvas.set_draw_color(Color::RGB(255, 255, 255));

        let p1_x = p1_pos1.round() as i32;
        let p1_y = p1_pos2.round() as i32;

        let p2_x = p2_pos1.round() as i32;
        let p2_y = p2_pos2.round() as i32;

        let _ = sdl.canvas.draw_line(Point::new(p1_x, p1_y), Point::new(p2_x, p2_y));
    }
}