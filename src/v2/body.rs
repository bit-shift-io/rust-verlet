/* 
use cgmath::Point2;
use collision::Aabb2;
*/
//use sdl2::pixels::Color;
//use sdl2::rect::Point;

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use cgmath::{InnerSpace, Vector2};

use crate::sdl_system::SdlSystem;

use super::attachment::Attachment;
use super::particle::Particle;
use super::stick::Stick;

pub struct Body {
    pub attachments: Vec<Rc<RefCell<Attachment>>>,
    pub particles: Vec<Rc<RefCell<Particle>>>,
    pub sticks: Vec<Rc<RefCell<Stick>>>,
    pub collides_with_self: bool,
    pub is_static: bool, // immovable?
    // collision_group(s) ?
    //pub aabb: Aabb2<f32>,
    pub gravity: Vector2<f32>,
    pub gravity_enabled: bool,
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
        Self { attachments: vec![], particles: vec![], sticks: vec![], collides_with_self: false, is_static: false, gravity: Vector2::new(0f32, 1000f32), gravity_enabled: true /* , aabb*/ }
    }

    pub fn set_gravity_enabled(&mut self, gravity_enabled: bool) {
        self.gravity_enabled = gravity_enabled;
    }

    pub fn set_gravity(&mut self, gravity: Vector2<f32>) {
        self.gravity = gravity;
    }

    pub fn set_collides_with_self(&mut self, collides_with_self: bool) {
        self.collides_with_self = collides_with_self;
    }

    pub fn set_static(&mut self, is_static: bool) {
        self.is_static = is_static;
    }

    pub fn add_attachment(&mut self, attachment: &Rc<RefCell<Attachment>>) {
        self.attachments.push(attachment.clone());
    }

    pub fn add_particle(&mut self, particle: &Rc<RefCell<Particle>>) {
        self.particles.push(particle.clone());
    }

    pub fn add_stick(&mut self, stick: &Rc<RefCell<Stick>>) {
        self.sticks.push(stick.clone());
    }

    pub fn pre_update(&mut self, dt: f32) {
        /* 
        if self.is_static {
            return;
        }
        
        self.add_gravity();
        */
    }

    pub fn post_update(&mut self, dt: f32) {
        if self.is_static {
            return;
        }

        self.update_positions(dt);
        self.update_attachments(dt);
    }

    pub fn zero_forces(&mut self) {
        for particle in self.particles.iter() {
            let mut p = particle.as_ref().borrow_mut();
            p.set_force(Vector2::new(0f32, 0f32));
        }
    }

    pub fn add_gravity(&mut self) {
        if !self.gravity_enabled {
            return;
        }

        for particle in self.particles.iter() {
            let mut p = particle.as_ref().borrow_mut();
            let f = p.acceleration_to_force(self.gravity);
            p.add_force(f);
        }
    }

    fn update_attachments(&mut self, dt: f32) {
        for attachment in self.attachments.iter() {
            attachment.as_ref().borrow_mut().update(dt);
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

        // draw attachments
        for attachment in self.attachments.iter() {
            attachment.as_ref().borrow().draw(sdl);
        }
    }

    pub fn add_rotational_force_around_point(&mut self, pos: Vector2<f32>, force: f32) {
        for particle in self.particles.iter() {
            let mut p = particle.as_ref().borrow_mut();
            let delta = p.pos - pos;
            let adjacent = Vector2::new(-delta[1], delta[0]); // compute a vector at 90 degress to delta
            //let dist = delta.normalize();
            //let dir = delta.cross
            //p.force += 
            //let f = p.acceleration_to_force(adjacent * force);
            p.add_force(adjacent * force);
        }

    }
}


#[cfg(test)]
mod tests {
    use sdl2::pixels::Color;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn add_rotational_force_around_point() {
        // Conventionally, positive angle measures describe counterclockwise rotations. If we want to describe a clockwise rotation, we use negative angle measures.
        // so a positive force should be applied counterclockwise also.

        let mut body = Body::new();

        let col = Color::RGB(0, 0, 0);
        let pos = Vector2::new(1f32, 0f32);
        let particle = Rc::new(RefCell::new(Particle::new(pos, 1f32, 1f32, col)));
        body.add_particle(&particle);

        body.add_rotational_force_around_point(Vector2::new(0f32, 0f32), 1000f32);
        assert_eq!(particle.as_ref().borrow().force, Vector2::new(0f32, 1000f32));

        // clear the force on the particle
        particle.as_ref().borrow_mut().set_force(Vector2::new(0f32, 0f32));

        body.add_rotational_force_around_point(Vector2::new(0f32, 0f32), -1000f32);
        assert_eq!(particle.as_ref().borrow().force, Vector2::new(0f32, -1000f32));


        // now lets move the particle to a different axis
        // to make sure forces in the other axis are applied correctly
        let pos2 = Vector2::new(0f32, 1f32);
        particle.as_ref().borrow_mut().pos = pos2;
        particle.as_ref().borrow_mut().set_force(Vector2::new(0f32, 0f32));

        body.add_rotational_force_around_point(Vector2::new(0f32, 0f32), 1000f32);
        assert_eq!(particle.as_ref().borrow().force, Vector2::new(-1000f32, 0f32));

        // clear the force on the particle
        particle.as_ref().borrow_mut().set_force(Vector2::new(0f32, 0f32));

        body.add_rotational_force_around_point(Vector2::new(0f32, 0f32), -1000f32);
        assert_eq!(particle.as_ref().borrow().force, Vector2::new(1000f32, 0f32));
    }
}