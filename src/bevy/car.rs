use bevy::{input::ButtonInput, prelude::{Component, KeyCode, Res}};
use sdl2::keyboard::Keycode;
use crate::v3::{particle_accelerator::{ParticleAccelerator, ParticleHandle, ParticleManipulator, WeightedParticle}, shape_builder::ShapeBuilder, types::Vec2};

use super::car_scene::{cm_to_m, g_to_kg, CarSceneContext};

pub struct CarWheel {
    hub_particle_handle: ParticleHandle,
    surface_particle_handles: Vec<ParticleHandle>,
    interior_particle_handles: Vec<ParticleHandle>,
}

impl CarWheel {
    pub fn new(origin: Vec2, particle_accelerator: &mut ParticleAccelerator) -> Self {
        let particle_mass = 1.0; //g_to_kg(10.0);

        // wheel hub - this is on mask layer zero which is a special no collisions layer
        let hub_particle_handle = {
            let mask = 0x0;
            let particle_radius = cm_to_m(4.0);
            let mut builder = ShapeBuilder::new();
            builder.set_mass(particle_mass);
            builder.add_particle(origin, particle_radius)
                .create_in_particle_accelerator(particle_accelerator, mask);
            builder.particle_handles.first().unwrap().clone()
        };

        // wheel surface
        let surface_particle_handles = {
            let mask = 0x1;
            let divisions = 12;
            let circle_radius = cm_to_m(20.0); // around a typical car tyre size - 17-18" (once you account for particle radius)
            let particle_radius = cm_to_m(5.0);
            let mut builder = ShapeBuilder::new();
            builder.set_mass(particle_mass);
            builder.add_adjacent_stick_circle(origin, circle_radius, particle_radius, divisions)
                .create_in_particle_accelerator(particle_accelerator, mask);
            builder.particle_handles.clone()
        };

        // wheel interior
        let interior_particle_handles = {
            let mask = 0x1;
            let divisions = 7;
            let circle_radius = cm_to_m(12.0);
            let particle_radius = cm_to_m(6.0);
            let mut builder = ShapeBuilder::new();
            builder.set_mass(particle_mass);
            builder.add_circle(origin, circle_radius, particle_radius, divisions)
                .create_in_particle_accelerator(particle_accelerator, mask);
            builder.particle_handles.clone()
            
            //vec![]
        };       


        // notes:
        // the wheel hub needs a constraint to set its position to the centre of the wheel
        // that is its position should be determined by a few points on the surface wheel.
        // that said, this might cause issues with the air inside the wheel (YES, this is happening!). If this is the case
        // we need a way to disable collisions for the hub (set radius to 0 - no we need to disable collision for the hub with the air - could use collision masks?). Set its layer to zero to mean the no collisions layer?
        // or add a flag to particles to say they are "invisible"?

         
        // to optimise this we really only need maybe 4 points to determine the centre of the wheel for the incoming particles
        // we set all particles as output particles so the axle can be pushed by any sticks
        let mut weighted_particles = vec![];
        for particle_handle in surface_particle_handles.iter() {
            weighted_particles.push(WeightedParticle::new(particle_handle.clone(), 1.0));
        }

        // todo: reenable outgoing_particles
        particle_accelerator.create_attachment_constraint(weighted_particles.clone(), weighted_particles.clone(), hub_particle_handle.clone());
        

        Self {
            hub_particle_handle,
            surface_particle_handles,
            interior_particle_handles
        }
    }

    fn rotate(&mut self, direction: f32, particle_accelerator: &mut ParticleAccelerator) {
        let centre = particle_accelerator.get_particle_position(&self.hub_particle_handle);
        let force_magnitude = 50.0;

        let particle_manipulator = ParticleManipulator::new();
        particle_manipulator.add_rotational_force_around_point(particle_accelerator, &self.surface_particle_handles, centre, force_magnitude * direction);
        particle_manipulator.add_rotational_force_around_point(particle_accelerator, &self.interior_particle_handles, centre, force_magnitude * direction);
    }
}

const NUM_WHEELS: usize = 2;

pub struct Car {
    pub wheels: [CarWheel; NUM_WHEELS],
}

impl Car {
    pub fn new(particle_accelerator: &mut ParticleAccelerator, origin: Vec2) -> Self {
        let wheel_spacing = 1.0 * 0.5; // metres

        let wheel_1 = CarWheel::new(origin + Vec2::new(wheel_spacing, 0.0), particle_accelerator);
        let wheel_2 = CarWheel::new(origin - Vec2::new(wheel_spacing, 0.0), particle_accelerator);

        
        // axle stick to connect the two wheel hubs
        {
            let length = (particle_accelerator.get_particle_position(&wheel_1.hub_particle_handle) - particle_accelerator.get_particle_position(&wheel_2.hub_particle_handle)).magnitude(); 
            particle_accelerator.create_stick([&wheel_1.hub_particle_handle, &wheel_2.hub_particle_handle], length, 0.0);
        }

        Self {
            wheels: [wheel_1, wheel_2],
        }
    }

    fn rotate_wheels(&mut self, direction: f32, particle_accelerator: &mut ParticleAccelerator) {
        for wheel in self.wheels.iter_mut() { 
            wheel.rotate(direction, particle_accelerator);
        }
    }

    pub fn update(&mut self, particle_accelerator: &mut ParticleAccelerator, keys: Res<ButtonInput<KeyCode>>) {
        if keys.pressed(KeyCode::KeyZ) {
            self.rotate_wheels(1.0, particle_accelerator); // ccw
        }
        if keys.pressed(KeyCode::KeyX) {
            self.rotate_wheels(-1.0, particle_accelerator); // clockwise
        }
    }

    pub fn get_camera_look_at_position(&self, particle_accelerator: &mut ParticleAccelerator, ) -> Vec2 {
        let mut pos = Vec2::new(0.0, 0.0);
        for wheel in self.wheels.iter() {
            pos += particle_accelerator.get_particle_position(&wheel.hub_particle_handle);
        }
        pos /= NUM_WHEELS as f32;
        //pos.extend(1.0); // homogeneous coordinate
        pos
    }
}