use bevy::math::Vec2;
use nalgebra::Vector2;
use sdl2::pixels::Color;

use crate::{v3::particle_accelerator::{ParticleAccelerator, SpringHandle, StickHandle}, v4::{particle_container::ParticleContainer, particle_handle::ParticleHandle}};

use super::super::particle::{Particle};

// Utility function that takes 2 points (a line segment) and a radius
// and calculates how many circles can fit touching each other between the 2 points.
pub fn radius_divisions_between_points(p1: Vec2, p2: Vec2, radius: f32) -> usize {
    let dist = (p2 - p1).length();
    let divisions = (dist / (radius * 2.0)) as usize;
    return divisions;
}

pub trait PositionProvider {
    fn get_points_for_radius(&self, radius: f32) -> Vec::<Vec2>;
}

pub struct LineSegment {
    p1: Vec2,
    p2: Vec2,
}

impl LineSegment {
    pub fn new(p1: Vec2, p2: Vec2) -> Self {
        Self { p1, p2 }
    }
}

impl PositionProvider for LineSegment {
    fn get_points_for_radius(&self, radius: f32) -> Vec::<Vec2> {
        let mut points = vec![];

        let divisions = radius_divisions_between_points(self.p1, self.p2, radius);
        let delta = self.p2 - self.p1;

        for i in 0..divisions { 
            let percent = i as f32 / divisions as f32;
            let pos = self.p1 + (delta * percent);
            points.push(pos);
        }

        points
    }
}

pub struct Circle {
    centre: Vec2,
    radius: f32,
}

impl Circle {
    pub fn new(centre: Vec2, radius: f32) -> Self {
        Self { centre, radius }
    }
}

impl PositionProvider for Circle {
    fn get_points_for_radius(&self, radius: f32) -> Vec::<Vec2> {
        let mut points = vec![];

        // putting a smaller circle on the bigger circle, creates 2x isosceles triangles where they intersect
        // so solve to find the half angle
        // https://www.quora.com/How-do-you-find-the-angles-of-an-isosceles-triangle-given-three-sides
        let top = radius * radius; // particle radius ^ 2
        let bottom = 2.0 * self.radius * self.radius; // circle_radius ^ 2
        let c_angle = f32::acos(1.0 - (top / bottom)); // this is the half angle made by the isosceles trangle from the 2 intersecting circles
        let theta = c_angle * 2.0;
        
        let divisions = (2.0 * std::f32::consts::PI) / theta;
        let integer_divisions = divisions as usize;
        for i in 0..integer_divisions {
            let radians = i as f32 * theta;
            let x = f32::sin(radians);
            let y = f32::cos(radians);
            let pos = self.centre + Vec2::new(x * self.radius, y * self.radius);
            points.push(pos);
        }

        points
    }
}

pub struct ShapeBuilder {
    pub particles: Vec<Particle>,

    pub particle_template: Particle,

    pub cursor: Vec2,
    /* 
    sticks: Vec<StickPrim>,
    springs: Vec<SpringPrim>,

    // particle properties
    is_static: bool,
    mass: f32,
    color: Color,
    radius: f32,
    stiffness_factor: f32,

    // spring properties
    spring_constant: f32,
    elastic_limit: f32,
    damping: f32,
*/
    pub particle_handles: Vec<ParticleHandle>,
    pub stick_handles: Vec<StickHandle>,
    pub spring_handles: Vec<SpringHandle>,
}

impl ShapeBuilder {
    pub fn new() -> Self {
        Self { 
            particles: vec![], 
            particle_template: Particle::default(),
            cursor: Vec2::new(0.0, 0.0),

            particle_handles: vec![],
            stick_handles: vec![],
            spring_handles: vec![],
        }    
    }

    pub fn set_particle_template(&mut self, particle_template: Particle) -> &mut Self {
        self.particle_template = particle_template;
        self
    }

    pub fn add_particle(&mut self, particle: Particle) -> &mut Self {
        self.particles.push(particle);
        self
    }

    // create a particle from the particle_template and place it at the given position
    // then add it
    pub fn add_particle_at_position(&mut self, pos: Vec2) -> &mut Self {
        let p = self.create_particle().set_position(pos).clone();
        self.add_particle(p);
        self
    }

    // create a particle from the particle_template
    pub fn create_particle(&mut self) -> Particle {
        self.particle_template.clone()
    }

    pub fn create_in_particle_container(&mut self, particle_container: &mut ParticleContainer) -> &mut Self {
        for particle in self.particles.iter() {
            let particle_handle = particle_container.add(*particle);
            self.particle_handles.push(particle_handle);
        }
        self
    }

    /* 
    pub fn create_in_particle_accelerator(&mut self, particle_accelerator: &mut ParticleAccelerator, mask: u32) -> &mut Self {
        let mut particle_handles = vec![];
        for particle in self.particles.iter() {

            // todo: shitty conversions, fix me!
            let math_pos = Vector2::<f32>::new(particle.pos.x, particle.pos.y);

            let linear = particle.color.to_linear();
            let sdl_color = Color::RGBA((linear.red * 255.0) as u8, (linear.green * 255.0) as u8, (linear.blue * 255.0) as u8, (linear.alpha * 255.0) as u8);

            let particle_handle = particle_accelerator.create_particle(math_pos, particle.radius, particle.mass, mask, sdl_color);
            particle_accelerator.set_particle_static(&particle_handle, particle.is_static);
            particle_handles.push(particle_handle);
        }
        self.particle_handles = particle_handles;

        /*
        let mut stick_handles = vec![];
        for stick in self.sticks.iter() {
            let stick_handle = particle_accelerator.create_stick([&particle_handles[stick.particle_indicies[0]], &particle_handles[stick.particle_indicies[1]]], stick.length, stick.stiffness_factor);
            stick_handles.push(stick_handle);
        }
        //self.stick_handles = stick_handles;

        let mut spring_handles = vec![];
        for spring in self.springs.iter() {
            let spring_handle = particle_accelerator.create_spring([&particle_handles[spring.particle_indicies[0]], &particle_handles[spring.particle_indicies[1]]], spring.length, spring.spring_constant, spring.damping, spring.elastic_limit);
            spring_handles.push(spring_handle);
        }*/
        //self.spring_handles = spring_handles;

        self
    }
    */

    pub fn add_particles(&mut self, position_provider: &dyn PositionProvider) -> &mut Self {
        let points = position_provider.get_points_for_radius(self.particle_template.radius);
        for p in points {
            self.add_particle_at_position(p);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec2;

    use super::*;

    #[test]
    fn set_particle_properties() {
        let mut b = ShapeBuilder::new();
        b.set_particle_template(Particle::default().set_radius(3.0).clone());
        assert_eq!(b.particle_template.radius, 3.0);
    }

    #[test]
    fn add_particle() {
        let mut b = ShapeBuilder::new();
        b.add_particle(Particle::default().set_position(Vec2::new(1.0, 1.0)).clone());
        assert_eq!(b.particles.len(), 1);
    }

    #[test]
    fn line() {
        let mut b = ShapeBuilder::new();
        b.add_particles(&LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0)));
        assert_eq!(b.particles.len(), 10);
    }

    #[test]
    fn create_in_particle_container() {
        let mut b = ShapeBuilder::new();
        b.set_particle_template(Particle::default().set_static(true).clone());
        b.add_particles(&Circle::new(Vec2::new(0.0, 0.0), 10.0));

        let mut pc = ParticleContainer::new();
        b.create_in_particle_container(&mut pc);

        assert_eq!(pc.particles.len(), b.particle_handles.len());
        assert_eq!(pc.particles.len(),  b.particles.len());
    }
}

