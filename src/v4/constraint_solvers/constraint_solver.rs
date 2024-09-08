use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use bevy::math::Vec2;

use crate::v4::constraint_container::ConstraintContainer;
use crate::v4::constraints::stick_constraint::StickConstraint;
use crate::v4::particle_solvers::particle_solver::compute_movement_weight;

use super::super::particle_container::ParticleContainer;

pub struct ConstraintSolver {
    particle_container: Arc<RwLock<ParticleContainer>>,
    constraint_container: Arc<RwLock<ConstraintContainer>>,
}

impl ConstraintSolver {
    pub fn new() -> Self {
        Self { 
            particle_container: Arc::new(RwLock::new(ParticleContainer::new())),
            constraint_container: Arc::new(RwLock::new(ConstraintContainer::new())),
        }
    }

    pub fn attach_to_containers(&mut self, particle_container: &Arc<RwLock<ParticleContainer>>, constraint_container: &Arc<RwLock<ConstraintContainer>>) {
        self.particle_container = particle_container.clone();
        self.constraint_container = constraint_container.clone();
    }

    pub fn notify_particle_container_changed(&mut self/* , particle_container: &Rc<RefCell<ParticleContainer>>, particle_index: usize*/) {
    }

    pub fn notify_constraint_container_changed(&mut self) {

    }

    pub fn update_constraints(&mut self, delta_seconds: f32) {
        //self.update_attachment_constraints(delta_seconds);
        self.update_sticks(delta_seconds);
        //self.update_springs(delta_seconds);
    }

    pub fn post_update_constraints(&mut self, delta_seconds: f32) {
        //self.post_update_attachment_constraints(delta_seconds);
    }


/* 
    fn update_attachment_constraints(&mut self, _delta_seconds: f32) {
        for attachment_constraint in particle_accelerator.attachment_constraints.iter_mut() {
            let mut pos = Vec2::new(0f32, 0f32);
            for weighted_particle in attachment_constraint.incoming_weighted_particles.iter() {
                let p = &particle_accelerator.verlet_positions[weighted_particle.particle_id];
                pos += p.pos * weighted_particle.weight;
            }
            pos /= attachment_constraint.incoming_weighted_particles.len() as f32;

            //let mut delta_pos = Vec2::new(0.0, 0.0);
            {
                let target_particle = &mut particle_accelerator.verlet_positions[attachment_constraint.target_particle_id];
                target_particle.pos = pos;

                // store the velocity of the target particle
                attachment_constraint.velocity_prev = target_particle.pos - target_particle.pos_prev;
                /* 
                delta_pos = pos - target_particle.pos_prev;


                if delta_pos.magnitude_squared() != 0.0 {
                    println!("axle moved by {}", delta_pos.magnitude());
                }
                if delta_pos.magnitude_squared().is_nan() {
                    println!("axle moved to infinity");
                }*/
            }
        }
    }

    fn post_update_attachment_constraints(&mut self, _delta_seconds: f32) {
        for attachment_constraint in particle_accelerator.attachment_constraints.iter_mut() {

            // the incoming particles (I) push the target particle (T) by X
            // so T has a given velocity.
            // now at the end of the frame, we see if there is any change in the velocity (i.e. acceleration)
            // and only apply the acceleration to the outgoing particles.
            //
            // if we just appliy the velocity to the outgoing particles (O) then we end up in a compounding
            // situation (at leaast for the case where incomming particles = outgoing particles)
            // and they zoom off to infinity as 
            // I push T, T push O, but I = O, so its a circular loop!
            // Only applying acceleration fixes this.
            //
            let mut delta_velocity = Vec2::new(0.0, 0.0);
            {
                let target_particle = &mut particle_accelerator.verlet_positions[attachment_constraint.target_particle_id];

                let current_velocity = target_particle.pos - target_particle.pos_prev;
                let prev_velocity = attachment_constraint.velocity_prev;

                delta_velocity = current_velocity - prev_velocity;

                /*
                if delta_velocity.magnitude_squared() != 0.0 {
                    println!("axle accelerated by {}", delta_velocity.magnitude());
                }
                if delta_velocity.magnitude_squared().is_nan() {
                    println!("axle moved to infinity");
                }*/
            }

            // push any outgoing particles based on their weight
            for weighted_particle in attachment_constraint.outgoing_weighted_particles.iter() {
                let p = &mut particle_accelerator.verlet_positions[weighted_particle.particle_id];
                p.pos += delta_velocity * weighted_particle.weight;
            }
        }
    }
*/

    fn update_sticks(&mut self, delta_seconds: f32) {
        let mut particle_container = self.particle_container.as_ref().write().unwrap();

        let mut constraint_container = self.constraint_container.as_ref().write().unwrap();
        for constraint in &constraint_container.constraints {
            //constraint.as_any().downcast_ref::<T>().unwrap()

            if let Some(stick) = constraint.as_any().downcast_ref::<StickConstraint>() {
                if !stick.is_enabled {
                    continue;
                }
    
                let particle_a = &particle_container.particles[stick.particle_handles[0].id()];
                let particle_b = &particle_container.particles[stick.particle_handles[1].id()];
    
                let (a_movement_weight, b_movement_weight) = compute_movement_weight(particle_a.is_static, particle_b.is_static);
                        
                //let p1 = &particle_container.verlet_positions[stick.particle_handles[0]];
                //let p2 = &particle_container.verlet_positions[stick.particle_handles[1]];
    
                let difference = particle_a.pos - particle_b.pos;
                let diff_length = difference.length(); //.magnitude();
                let diff_factor = (stick.length - diff_length) / diff_length * 0.5;
                let mut offset = (difference * diff_factor);
                
                // this bit makes it more like a spring
                if stick.stiffness_factor != 0.0 {
                    offset *= (delta_seconds * stick.stiffness_factor);
                }
    
        
                {
                    let p1mut = &mut particle_container.particles[stick.particle_handles[0].id()];
                    p1mut.pos += offset * a_movement_weight;
                }
    
                {
                    let p2mut = &mut particle_container.particles[stick.particle_handles[1].id()];
                    p2mut.pos -= offset * b_movement_weight;
                }
            }
        }

/* 
        for stick in particle_accelerator.sticks.iter_mut() {
            if !stick.is_enabled {
                continue;
            }

            let particle_a = &particle_accelerator.particles[stick.particle_indicies[0]];
            let particle_b = &particle_accelerator.particles[stick.particle_indicies[1]];

            let (a_movement_weight, b_movement_weight) = self.compute_movement_weight(particle_a.is_static, particle_b.is_static);
                    
            let p1 = &particle_accelerator.verlet_positions[stick.particle_indicies[0]];
            let p2 = &particle_accelerator.verlet_positions[stick.particle_indicies[1]];

            let difference = p1.pos - p2.pos;
            let diff_length = difference.magnitude();
            let diff_factor = (stick.length - diff_length) / diff_length * 0.5;
            let mut offset = (difference * diff_factor);
            
            // this bit makes it more like a spring
            if stick.stiffness_factor != 0.0 {
                offset *= (dt * stick.stiffness_factor);
            }

    
            {
                let p1mut = &mut particle_accelerator.verlet_positions[stick.particle_indicies[0]];
                p1mut.pos += offset * a_movement_weight;
            }

            {
                let p2mut = &mut particle_accelerator.verlet_positions[stick.particle_indicies[1]];
                p2mut.pos -= offset * b_movement_weight;
            }
        }*/
    }
/* 

    fn update_springs(&mut self, particle_accelerator: &mut ParticleAccelerator, dt: f32) {
        let mut i = 0;
        for spring in particle_accelerator.springs.iter_mut() {
            if !spring.is_enabled {
                continue;
            }

            let particle_a = &particle_accelerator.particles[spring.particle_indicies[0]];
            let particle_b = &particle_accelerator.particles[spring.particle_indicies[1]];

            let (a_movement_weight, b_movement_weight) = self.compute_movement_weight(particle_a.is_static, particle_b.is_static);
                    
            let p1 = &particle_accelerator.verlet_positions[spring.particle_indicies[0]];
            let p2 = &particle_accelerator.verlet_positions[spring.particle_indicies[1]];

            // https://jsfiddle.net/odestcj/g72MA/
            let difference = p2.pos - p1.pos;
            let current_length = difference.magnitude();
            let dir = difference.normalize();

            let extension = current_length - spring.length; // x
            let spring_force = -spring.spring_constant * extension; // hook's law

            //spring_force = spring_force * spring_force;

            // compute the velocity of the 2 particles
            let v1 = p1.pos - p1.pos_prev;
            let v2 =  p2.pos - p2.pos_prev;
            let velocity = v1 + v2;

            let damped_velocity = -(velocity * spring.damping);
            
            let damped_force = (dir * spring_force) + damped_velocity;
/* 
            let difference = p2.pos - p1.pos;
            let current_length = difference.magnitude();

            let dir = difference.normalize();

            let extension = current_length - spring.length;



            // F = (-k * x) - (velocity * damping_coefficient)
            let damped_velocity = (velocity * spring.damping);
            let force = (-dir * extension * spring.spring_constant);
            let damped_force = force;// - damped_velocity;

            //let diff_factor = (spring.length - diff_length) / diff_length * 0.5;
            //let offset = difference * diff_factor * spring.spring_constant;
    */

            /*
            if i == 0 && extension != 0.0 {
                println!("e: {},    sf: {}", extension, spring_force);
            }*/

            
            {
                let p1mut = &mut particle_accelerator.verlet_positions[spring.particle_indicies[0]];
                p1mut.force -= damped_force * a_movement_weight;
            }

            {
                let p2mut = &mut particle_accelerator.verlet_positions[spring.particle_indicies[1]];
                p2mut.force += damped_force * b_movement_weight;
            }

            /* 
            // lets test a stick spring hybrid that ignores mass, and moves based on extension
            let offset = (dir * extension * 0.5) * (dt * 0.8);

            {
                let p1mut = &mut particle_accelerator.verlet_positions[spring.particle_indicies[0]];
                p1mut.pos += offset * a_movement_weight;
            }

            {
                let p2mut = &mut particle_accelerator.verlet_positions[spring.particle_indicies[1]];
                p2mut.pos -= offset * b_movement_weight;
            }*/

            i += 1;
        }
    }*/
}