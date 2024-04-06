/* 
use cgmath::Point2;
use collision::Aabb2;
*/
//use sdl2::pixels::Color;
//use sdl2::rect::Point;

use std::cell::RefCell;
use std::rc::Rc;

use cgmath::Vector2;

use crate::sdl_system::SdlSystem;

use super::particle::Particle;
use super::stick::Stick;

pub struct Body {
    pub particles: Vec<Rc<RefCell<Particle>>>,
    pub sticks: Vec<Rc<RefCell<Stick>>>,
    pub collides_with_self: bool,
    pub is_static: bool, // immovable?
    // collision_group(s) ?
    //pub aabb: Aabb2<f32>,
    pub gravity: Vector2<f32>,
}

/* 
// https://github.com/rustgd/collision-rs/issues/138
fn aabb2(minx: f32, miny: f32, maxx: f32, maxy: f32) -> Aabb2<f32> {
    Aabb2::new(Point2::new(minx, miny), Point2::new(maxx, maxy))
}
*/

impl Body {
    pub fn new() -> Self {
        //let zero_point: Point<f32> = Point2::<f32>::new(0.0f32, 0.0f32);
        //let aabb: Aabb2<f32> = Aabb2::new(zero_point, zero_point);
        Self { particles: vec![], sticks: vec![], collides_with_self: false, is_static: false, gravity: Vector2::new(0f32, 1000f32) /* , aabb*/ }
    }

    pub fn set_static(&mut self, is_static: bool) {
        self.is_static = is_static;
    }

    pub fn add_particle(&mut self, particle: Rc<RefCell<Particle>>) {
        self.particles.push(particle);
    }

    pub fn add_stick(&mut self, stick: Rc<RefCell<Stick>>) {
        self.sticks.push(stick);
    }

    pub fn pre_update(&mut self, dt: f32) {
        if self.is_static {
            return;
        }
        
        self.apply_gravity();
    }

    pub fn post_update(&mut self, dt: f32) {
        if self.is_static {
            return;
        }

        self.update_positions(dt);
    }

    fn apply_gravity(&mut self) {
        for particle in self.particles.iter() {
            particle.as_ref().borrow_mut().accelerate(self.gravity);
        }
    }

    fn update_positions(&mut self, dt: f32) {
        for particle in self.particles.iter() {
            particle.as_ref().borrow_mut().update_position(dt);
        }
    }

    pub fn update_sticks(&mut self, dt: f32) {
        for stick in self.sticks.iter() {
            stick.as_ref().borrow_mut().update(dt);
        }
    }

    pub fn draw(&self, sdl: &mut SdlSystem) {
        // draw particles
        for particle in self.particles.iter() {
            particle.as_ref().borrow().draw(sdl);
        }

        // draw stick constraints
        for stick in self.sticks.iter() {
            stick.as_ref().borrow().draw(sdl);
        }
    }
}