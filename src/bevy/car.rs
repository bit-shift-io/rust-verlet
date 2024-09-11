use bevy::{color::{Color, LinearRgba}, input::ButtonInput, math::{vec2, Vec2}, prelude::{Component, KeyCode, Res}};

use crate::v4::{constraints::stick_constraint::StickConstraint, particle::Particle, particle_handle::ParticleHandle, particle_manipulator::ParticleManipulator, particle_sim::ParticleSim, shape_builder::{adjacent_sticks::AdjacentSticks, circle::Circle, shape_builder::ShapeBuilder}};

use super::car_scene::{cm_to_m, g_to_kg, CarSceneContext};

pub struct CarWheel {
    hub_particle_handle: ParticleHandle,
    surface_particle_handles: Vec<ParticleHandle>,
    //interior_particle_handles: Vec<ParticleHandle>,
}

impl CarWheel {
    pub fn new(origin: Vec2, particle_sim: &mut ParticleSim) -> Self {
        let particle_mass = 1.0; //g_to_kg(10.0);

        // wheel hub - this is on mask layer zero which is a special no collisions layer
        let hub_particle_handle = {
            let mask = 0x0;
            let particle_radius = cm_to_m(4.0);
            let mut builder = ShapeBuilder::new();
            builder.set_particle_template(Particle::default().set_mass(particle_mass).set_radius(particle_radius).set_color(Color::from(LinearRgba::GREEN)).clone());

            builder.add_particle(builder.create_particle().set_position(origin).clone())
                .create_in_particle_sim(particle_sim);

            builder.particle_handles.first().unwrap().clone()
        };

        // wheel surface
        let surface_particle_handles = {
            let mask = 0x1;
            let divisions = 20;
            let circle_radius = cm_to_m(35.0); // around a typical car tyre size - 17-18" (once you account for particle radius)
            let particle_radius = cm_to_m(4.0);
            let mut builder = ShapeBuilder::new();
            builder.set_particle_template(Particle::default().set_mass(particle_mass).set_radius(particle_radius).set_color(Color::from(LinearRgba::GREEN)).clone());

            builder.apply_operation(Circle::new(origin, circle_radius));
            builder.apply_operation(AdjacentSticks::new(StickConstraint::default().clone(), 1, true));
            builder.apply_operation(AdjacentSticks::new(StickConstraint::default().clone(), 4, true));

            builder.create_in_particle_sim(particle_sim);

            builder.particle_handles.clone()
        };

        /*
        // wheel interior
        let interior_particle_handles = {
            let mask = 0x1;
            let divisions = 14;
            let circle_radius = cm_to_m(14.0);
            let particle_radius = cm_to_m(4.0);
            let mut builder = ShapeBuilder::new();
            builder.set_mass(particle_mass);
            builder.add_circle(origin, circle_radius, particle_radius, divisions)
                //.connect_with_stick_chain(2) // stop the air escaping so easily
                .create_in_particle_sim(particle_sim, mask);
            builder.particle_handles.clone()
            
            //vec![]
        };       */


        // notes:
        // the wheel hub needs a constraint to set its position to the centre of the wheel
        // that is its position should be determined by a few points on the surface wheel.
        // that said, this might cause issues with the air inside the wheel (YES, this is happening!). If this is the case
        // we need a way to disable collisions for the hub (set radius to 0 - no we need to disable collision for the hub with the air - could use collision masks?). Set its layer to zero to mean the no collisions layer?
        // or add a flag to particles to say they are "invisible"?

         
         /* todo: port to v4
        // to optimise this we really only need maybe 4 points to determine the centre of the wheel for the incoming particles
        // we set all particles as output particles so the axle can be pushed by any sticks
        let mut weighted_particles = vec![];
        for particle_handle in surface_particle_handles.iter() {
            weighted_particles.push(WeightedParticle::new(particle_handle.clone(), 1.0));
        }

        // todo: reenable outgoing_particles
        particle_sim.create_attachment_constraint(weighted_particles.clone(), weighted_particles.clone(), hub_particle_handle.clone());
        */

        Self {
            hub_particle_handle,
            surface_particle_handles,
            //interior_particle_handles
        }
    }

    fn rotate(&mut self, direction: f32, particle_sim: &mut ParticleSim) {
        
        let hub_particle = particle_sim.get_particle(self.hub_particle_handle);
        let centre = hub_particle.pos;
        let force_magnitude = 60.0;

        let particle_manipulator = ParticleManipulator::new();
        particle_manipulator.add_rotational_force_around_point(particle_sim, &self.surface_particle_handles, centre, force_magnitude * direction);
        //particle_manipulator.add_rotational_force_around_point(particle_sim, &self.interior_particle_handles, centre, force_magnitude * direction);
    }
}

const NUM_WHEELS: usize = 2;

pub struct Car {
    pub wheels: [CarWheel; NUM_WHEELS],
}

impl Car {
    pub fn new(particle_sim: &mut ParticleSim, origin: Vec2) -> Self {
        let wheel_spacing = 1.0 * 0.5; // metres

        let wheel_1 = CarWheel::new(origin + Vec2::new(wheel_spacing, 0.0), particle_sim);
        let wheel_2 = CarWheel::new(origin - Vec2::new(wheel_spacing, 0.0), particle_sim);

        /* todo: port to v4
        
        // axle stick to connect the two wheel hubs
        {
            let length = (particle_sim.get_particle_position(&wheel_1.hub_particle_handle) - particle_sim.get_particle_position(&wheel_2.hub_particle_handle)).magnitude(); 
            particle_sim.create_stick([&wheel_1.hub_particle_handle, &wheel_2.hub_particle_handle], length, 0.0);
        }

        */
        Self {
            wheels: [wheel_1, wheel_2],
        }
    }

    fn rotate_wheels(&mut self, direction: f32, particle_sim: &mut ParticleSim) {
        for wheel in self.wheels.iter_mut() { 
            wheel.rotate(direction, particle_sim);
        }
    }

    pub fn update(&mut self, particle_sim: &mut ParticleSim, keys: Res<ButtonInput<KeyCode>>) {
        if keys.pressed(KeyCode::KeyZ) {
            self.rotate_wheels(1.0, particle_sim); // ccw
        }
        if keys.pressed(KeyCode::KeyX) {
            self.rotate_wheels(-1.0, particle_sim); // clockwise
        }
    }

    pub fn get_camera_look_at_position(&self, particle_sim: &ParticleSim) -> Vec2 {
        let mut pos = Vec2::new(0.0, 0.0);
        
        for wheel in self.wheels.iter() {
            pos += particle_sim.get_particle(wheel.hub_particle_handle).pos;
        }
        pos /= NUM_WHEELS as f32;
        //pos.extend(1.0); // homogeneous coordinate
        
        pos
    }
}