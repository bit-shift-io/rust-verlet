use bevy::math::{bounding::Aabb2d, vec2, Rot2, Vec2};
use collision::Aabb3;

use crate::v4::{constraint_container::ConstraintContainer, constraints::{constraint::Constraint, stick_constraint::StickConstraint}, particle::{self, Particle}, particle_container::ParticleContainer, particle_handle::{ConstraintHandle, ParticleHandle}};

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

pub trait ShapeBuilderOperation {
    fn apply_to_shape_builder(&self, shape_builder: &mut ShapeBuilder);
}

pub struct ShapeBuilder {
    pub particles: Vec<Particle>,
    pub particle_template: Particle,

    pub constraints: Vec<Box<dyn Constraint + Send + Sync>>,
    pub constraint_template: Box<dyn Constraint>,
    

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
    //pub stick_handles: Vec<StickHandle>,
    pub constraint_handles: Vec<ConstraintHandle>,
}

impl ShapeBuilder {
    pub fn new() -> Self {
        Self { 
            particles: vec![], 
            particle_template: Particle::default(),

            constraints: vec![],
            constraint_template: Box::new(StickConstraint::default()),

            cursor: Vec2::new(0.0, 0.0),

            particle_handles: vec![],
            constraint_handles: vec![],

            //stick_handles: vec![],
            //spring_handles: vec![],
        }    
    }

    pub fn from_shape_builder_templates(sb: &ShapeBuilder) -> Self {
        let mut new_sb = ShapeBuilder::new();
        new_sb.set_particle_template(sb.particle_template);

        //new_sb.set_constraint_template(sb.constraint_template.as_ref().clone());
        new_sb.constraint_template = sb.constraint_template.box_clone(); //Box::new(*sb.constraint_template.as_ref());
        
        new_sb
    }

    pub fn set_constraint_template<T: Constraint + 'static>(&mut self, constraint_template: T) -> &mut Self {
        self.constraint_template = Box::new(constraint_template);
        self
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

    pub fn create_in_constraint_container(&mut self, constraint_container: &mut ConstraintContainer) -> &mut Self {
        for constraint in self.constraints.iter() {
            let constraint_handle = constraint_container.add(constraint.box_clone());
            self.constraint_handles.push(constraint_handle);
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

    /* 
    pub fn add_particles(&mut self, particles: Vec<Particle>) -> &mut Self {
        for p in particles {
            self.particles.push(p);
        }
        self
    }*/

    pub fn particle_radius(&self) -> f32 {
        self.particle_template.radius
    }

    pub fn apply_operation(&mut self, operation: &dyn ShapeBuilderOperation) -> &mut Self {
        operation.apply_to_shape_builder(self);
        self
    }

    pub fn add_particles_from_points(&mut self, points: &Vec<Vec2>) -> &mut Self {
        for p in points {
            self.add_particle_at_position(p.clone());
        }
        self
    }

    /*
    pub fn add_particles(&mut self, position_provider: &dyn PositionProvider) -> &mut Self {
        let points = position_provider.get_points_for_radius(self.particle_template.radius);
        for p in points {
            self.add_particle_at_position(p);
        }
        self
    }*/

    pub fn get_aabb(&self) -> Aabb2d {
        // extracted from Aabb2d::from_point_cloud
        let mut points_iter = self.particles.iter().map(|particle| particle.pos);//.collect::<Vec<Vec2>>();//.try_into().unwrap();
        
        let first = points_iter
            .next()
            .expect("point cloud must contain at least one point for Aabb2d construction");

        let (min, max) = points_iter.fold((first, first), |(prev_min, prev_max), point| {
            (point.min(prev_min), point.max(prev_max))
        });

        Aabb2d {
            min,
            max
        }
    }

    pub fn extract_left_most_particles(&mut self) -> ShapeBuilder {
        let aabb = self.get_aabb();

        let mut s = ShapeBuilder::from_shape_builder_templates(&self);

        let r = self.particles.extract_if(|particle| particle.pos.x == aabb.min.x).collect::<Vec<_>>();
        //s.add_particles(r); // todo: add a fn for this
        for particle in r {
            s.add_particle(particle);
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec2;

    use crate::v4::shape_builder::{circle::Circle, line_segment::LineSegment};

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
        b.apply_operation(&LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0)));
        assert_eq!(b.particles.len(), 10);
    }

    #[test]
    fn create_in_particle_container() {
        let mut b = ShapeBuilder::new();
        b.set_particle_template(Particle::default().set_static(true).clone());
        b.apply_operation(&Circle::new(Vec2::new(0.0, 0.0), 10.0));

        let mut pc = ParticleContainer::new();
        b.create_in_particle_container(&mut pc);

        assert_eq!(pc.particles.len(), b.particle_handles.len());
        assert_eq!(pc.particles.len(),  b.particles.len());
    }
}

