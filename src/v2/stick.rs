use std::{cell::RefCell, rc::Rc};

use cgmath::InnerSpace;
use sdl2::{pixels::Color, rect::Point};

use crate::{point::{vec2_to_point, vec2_to_point_old}, sdl_system::SdlSystem};
use super::{particle::Particle, position::Position};

pub struct Stick {
    pub length: f32,
    pub p1: Rc<RefCell<dyn Position>>,
    pub p2: Rc<RefCell<dyn Position>>,
    //pub color: Color,
}

impl Stick {
    pub fn new(p1: &Rc<RefCell<dyn Position>>, p2: &Rc<RefCell<dyn Position>>) -> Self {
        let pos1 = p1.as_ref().borrow().get_position();
        let pos2 = p2.as_ref().borrow().get_position();
        let length = (pos1 - pos2).magnitude();
        Self { length, p1: p1.clone(), p2: p2.clone() }
    }

    pub fn update(&mut self, dt: f32) {

        let p1_new_pos;
        let p2_new_pos;

        {
            let p1 = self.p1.as_ref().borrow();
            let p2 = self.p2.as_ref().borrow();

            let p1_pos = p1.get_position();
            let p2_pos = p2.get_position();

            // temporary - do some checking
            //let pos_x = i16::try_from(p1_pos[0].round() as i32).unwrap();
            //let pos_y = i16::try_from(p1_pos[1].round() as i32).unwrap();

            let difference = p1_pos - p2_pos;
            let diff_length = difference.magnitude();
            let diff_factor = (self.length - diff_length) / diff_length * 0.5;

            let offset = difference * diff_factor;

            p1_new_pos = p1_pos + offset;
            p2_new_pos = p2_pos - offset;
        }

        {
            // temporary - do some checking
            //let pos_x = i16::try_from(p1_new_pos[0].round() as i32).unwrap();
            //let pos_y = i16::try_from(p1_new_pos[1].round() as i32).unwrap();

            let mut p1mut = self.p1.as_ref().borrow_mut();
            //println!("set position 1 ({}, {})", p1_new_pos[0], p1_new_pos[1]);
            p1mut.set_position(p1_new_pos);
        }

        {
            // temporary - do some checking
            //let pos_x = i16::try_from(p2_new_pos[0].round() as i32).unwrap();
            //let pos_y = i16::try_from(p2_new_pos[1].round() as i32).unwrap();

            let mut p2mut = self.p2.as_ref().borrow_mut();
            //println!("set position 2 ({}, {})", p2_new_pos[0], p2_new_pos[1]);
            p2mut.set_position(p2_new_pos);
        }
    }

    pub fn draw(&self, sdl: &mut SdlSystem) {
        let start = vec2_to_point_old(self.p1.as_ref().borrow().get_position());
        let end = vec2_to_point_old(self.p2.as_ref().borrow().get_position());

        sdl.canvas.set_draw_color(Color::RGB(255, 255, 255));
        let _ = sdl.canvas.draw_line(start, end);
    }
}