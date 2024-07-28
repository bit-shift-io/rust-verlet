use std::time::Duration;

use super::{particle_accelerator::ParticleAccelerator, types::Vec2};


/// Handles the moving and colliding of particles and constraints
pub struct ParticleCollider {

}

impl ParticleCollider {
    pub fn new() -> Self {
        Self {}
    }

    fn compute_movement_weight(&self, a_is_static: bool, b_is_static: bool) -> (f32, f32) {
        // movement weight is used to stop static objects being moved
        let a_movement_weight = if a_is_static { 0.0f32 } else if b_is_static { 1.0f32 } else { 0.5f32 };
        let b_movement_weight = 1.0f32 - a_movement_weight;
        (a_movement_weight, b_movement_weight)
    }

    // dt = last frame elapsed time
    // desired_hertz = times per second
    pub fn range_substeps_2(&self, elapsed_seconds: f32, desired_hertz: f32) -> Vec<f32> {
        //let last_elapsed_secs = last_elapsed.as_secs_f32();
        let substeps: f32 = elapsed_seconds * desired_hertz as f32;
        let rounded_substeps = substeps.floor() as usize;
        let increment = 1.0 / desired_hertz;
        vec![increment; rounded_substeps]
    }

    pub fn range_substeps(&self, dt: f32, substeps: usize) -> Vec<f32> {
        let sub_dt: f32 = dt / substeps as f32;
        vec![sub_dt; substeps]
    }

    pub fn acceleration_to_force(acc: Vec2, mass: f32) -> Vec2 {
        acc * mass
    }

    pub fn reset_forces(&mut self, particle_accelerator: &mut ParticleAccelerator, gravity: Vec2) {
        for verlet_position in particle_accelerator.verlet_positions.iter_mut() {
            let force = Self::acceleration_to_force(gravity, verlet_position.mass);
            verlet_position.force = force;
        }
    }

    pub fn solve_collisions(&mut self, particle_accelerator: &mut ParticleAccelerator) {
        for layer in particle_accelerator.layer_map.values() {
            // for each layer, we need to collide with each particle
            let particle_count: usize = layer.particle_ids.len();
            for ai in 0..particle_count {
                for bi in (&ai+1)..particle_count {
                    let particle_id_a = layer.particle_ids[ai];
                    let particle_id_b = layer.particle_ids[bi];
                   
                    let particle_a = &particle_accelerator.particles[particle_id_a];
                    let particle_b = &particle_accelerator.particles[particle_id_b];

                    // ignore static - static collisions
                    if particle_a.is_static && particle_b.is_static {
                        continue;
                    }

                    // ignore disabled particles
                    if !particle_a.is_enabled || !particle_b.is_enabled {
                        continue;
                    }

                    let (a_movement_weight, b_movement_weight) = self.compute_movement_weight(particle_a.is_static, particle_b.is_static);
                    
                    let collision_axis: Vec2;
                    let dist: f32;
                    let min_dist: f32;

                    // in a code block so ap and bp borrows are released as we need to borrow mut later if
                    // there is a collision
                    {
                        //let ap = a_particle.as_ref().borrow();
                        //let bp = b_particle.as_ref().borrow();
                        let verlet_position_a = &particle_accelerator.verlet_positions[particle_id_a];
                        let verlet_position_b = &particle_accelerator.verlet_positions[particle_id_b];
                    
                        collision_axis = verlet_position_a.pos - verlet_position_b.pos;
                        dist = (collision_axis[0].powf(2f32) + collision_axis[1].powf(2f32)).sqrt();
                        min_dist = particle_a.radius + particle_b.radius;
                    }

                    if dist < min_dist as f32 {
                        let n: Vec2 = collision_axis / dist;
                        let delta: f32 = min_dist as f32 - dist;

                        // is it better to have no if statement to make the loop tight at the cost
                        // of wasted vector computations?
                        //let mut ap_mut = a_particle.as_ref().borrow_mut();
                        let verlet_position_a = &mut particle_accelerator.verlet_positions[particle_id_a];
                        verlet_position_a.pos += a_movement_weight * delta * n;

                        //let mut bp_mut = b_particle.as_ref().borrow_mut();
                        let verlet_position_b = &mut particle_accelerator.verlet_positions[particle_id_b];
                        verlet_position_b.pos -= b_movement_weight * delta * n;
                    }
                }
            }
        }
    }

    pub fn update_positions(&mut self, particle_accelerator: &mut ParticleAccelerator, dt: f32) {
        let mut i = 0;
        for (particle, verlet_position) in particle_accelerator.particles.iter().zip(particle_accelerator.verlet_positions.iter_mut()) {
            /*
            if i == 65 {
                println!("65!");
            }*/
            if particle.is_static || !particle.is_enabled {
                continue
            }

            let velocity: Vec2 = verlet_position.pos - verlet_position.pos_prev;
            let acceleration: Vec2 = verlet_position.force / verlet_position.mass;
            verlet_position.pos_prev = verlet_position.pos;
            verlet_position.pos = verlet_position.pos + velocity + acceleration * dt * dt;

            i += 1;
        }
    }

    pub fn update_constraints(&mut self, particle_accelerator: &mut ParticleAccelerator, dt: f32) {
        self.update_attachment_constraints(particle_accelerator, dt);
        self.update_sticks(particle_accelerator, dt);
        self.update_springs(particle_accelerator, dt);
    }

    pub fn post_update_constraints(&mut self, particle_accelerator: &mut ParticleAccelerator, dt: f32) {
        self.post_update_attachment_constraints(particle_accelerator, dt);
    }

    pub fn update_attachment_constraints(&mut self, particle_accelerator: &mut ParticleAccelerator, _dt: f32) {
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

    pub fn post_update_attachment_constraints(&mut self, particle_accelerator: &mut ParticleAccelerator, _dt: f32) {
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

    pub fn update_sticks(&mut self, particle_accelerator: &mut ParticleAccelerator, dt: f32) {
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
        }
    }


    pub fn update_springs(&mut self, particle_accelerator: &mut ParticleAccelerator, dt: f32) {
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
    }
}
