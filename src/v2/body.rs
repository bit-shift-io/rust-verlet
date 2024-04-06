/* 
use cgmath::Point2;
use collision::Aabb2;
*/
//use sdl2::pixels::Color;
//use sdl2::rect::Point;

use std::cell::RefCell;
use std::rc::Rc;

use crate::sdl_system::SdlSystem;

use super::particle::Particle;
use super::stick::Stick;

pub struct Body {
    pub particles: Vec<Rc<RefCell<Particle>>>,
    pub sticks: Vec<Rc<RefCell<Stick>>>,
    pub collides_with_self: bool,
    // collision_group(s) ?
    //pub aabb: Aabb2<f32>,
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
        Self { particles: vec![], sticks: vec![], collides_with_self: false /* , aabb*/ }
    }

    pub fn add_particle(&mut self, particle: Rc<RefCell<Particle>>) {
        self.particles.push(particle);
    }

    pub fn add_stick(&mut self, stick: Rc<RefCell<Stick>>) {
        self.sticks.push(stick);
    }

    pub fn update(&mut self, dt: f32) {
        /* 
        self.apply_gravity();
        self.apply_containment_constraint();
        self.solve_collisions(sub_dt);
        self.update_positions(sub_dt);
        */
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