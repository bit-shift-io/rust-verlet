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

    pub fn update(&mut self, dt: f32) {
        let p1_pos = self.p1.as_ref().borrow().pos;
        let p2_pos = self.p2.as_ref().borrow().pos;

        let difference = p1_pos - p2_pos;
        let diff_length = difference.magnitude();
        let diff_factor = (self.length - diff_length) / diff_length * 0.5;
        let offset = difference * diff_factor;

        {
            let mut p1mut = self.p1.as_ref().borrow_mut();
            p1mut.pos += offset;
        }

        {
            let mut p2mut = self.p2.as_ref().borrow_mut();
            p2mut.pos -= offset;
        }
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