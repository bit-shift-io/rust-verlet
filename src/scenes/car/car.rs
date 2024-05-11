use std::{cell::RefCell, rc::Rc};

use cgmath::{InnerSpace, Vector2};
use sdl2::{event::Event, gfx::primitives::DrawRenderer, keyboard::Keycode, pixels::Color};
use rand::Rng;

use crate::{application::{Context, Scene}, keyboard::Keyboard, mouse::Mouse, v2::{attachment::Attachment, body::Body, particle::Particle, position::Position, solver::Solver, stick::Stick}, v3::{particle_accelerator::{self, ParticleAccelerator, ParticleHandle, ParticleManipulator, WeightedParticle}, shape_builder::ShapeBuilder, types::Vec2}};

use super::car_scene::{self, CarSceneContext};

pub struct CarWheel {
    hub_particle_handle: ParticleHandle,
    surface_particle_handles: Vec<ParticleHandle>,
    interior_particle_handles: Vec<ParticleHandle>,
}

impl CarWheel {
    pub fn new(origin: Vec2, particle_accelerator: &mut ParticleAccelerator) -> Self {
        
        // wheel hub - this is on mask layer zero which is a special no collisions layer
        let hub_particle_handle = {
            let mask = 0x0;
            let particle_radius = 4.0;
            let mut builder = ShapeBuilder::new();
            builder.add_particle(origin, particle_radius)
                .create_in_particle_accelerator(particle_accelerator, mask);
            builder.particle_handles.first().unwrap().clone()
        };

        // wheel surface
        let surface_particle_handles = {
            let mask = 0x1;
            let divisions = 8;
            let circle_radius = 20.0;
            let particle_radius = 7.0;
            let mut builder = ShapeBuilder::new();
            builder.add_adjacent_stick_circle(origin, circle_radius, particle_radius, divisions)
                .create_in_particle_accelerator(particle_accelerator, mask);
            builder.particle_handles.clone()
        };

        // wheel interior
        let interior_particle_handles = {
            let mask = 0x1;
            let divisions = 6;
            let circle_radius = 10.0;
            let particle_radius = 4.0;
            let mut builder = ShapeBuilder::new();
            builder.add_circle(origin, circle_radius, particle_radius, divisions)
                .create_in_particle_accelerator(particle_accelerator, mask);
            builder.particle_handles.clone()
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
        particle_accelerator.create_attachment_constraint(weighted_particles.clone(), vec![]/*weighted_particles.clone()*/, hub_particle_handle.clone());
        

        Self {
            hub_particle_handle,
            surface_particle_handles,
            interior_particle_handles
        }
    }

    fn rotate(&mut self, direction: f32, car_scene_context: &mut CarSceneContext) {
        let centre = car_scene_context.particle_accelerator.get_particle_position(&self.hub_particle_handle);
        let force_magnitude = 50f32;

        let particle_manipulator = ParticleManipulator::new();
        particle_manipulator.add_rotational_force_around_point(car_scene_context.particle_accelerator, &self.surface_particle_handles, centre, force_magnitude * direction);
        particle_manipulator.add_rotational_force_around_point(car_scene_context.particle_accelerator, &self.interior_particle_handles, centre, force_magnitude * direction);
    }
}

const NUM_WHEELS: usize = 2;

pub struct Car {
    pub wheels: [CarWheel; NUM_WHEELS],
}

impl Car {
    pub fn new(particle_accelerator: &mut ParticleAccelerator) -> Self {
        let wheel_1 = CarWheel::new(Vec2::new(300.0f32, 300.0f32), particle_accelerator);
        let wheel_2 = CarWheel::new(Vec2::new(400.0f32, 300.0f32), particle_accelerator);

        
        // axle stick to connect the two wheel hubs
        {
            let length = (particle_accelerator.get_particle_position(&wheel_1.hub_particle_handle) - particle_accelerator.get_particle_position(&wheel_2.hub_particle_handle)).magnitude(); 
            particle_accelerator.create_stick([&wheel_1.hub_particle_handle, &wheel_2.hub_particle_handle], length);
        }

        Self {
            wheels: [wheel_1, wheel_2],
        }
    }

    fn rotate_wheels(&mut self, direction: f32, car_scene_context: &mut CarSceneContext) {
        // something wrong here, z should be counterclockwise. x should be clockwise
        // be it seems reversed!? because in SDL the y-axis is mirrored around the x-axis
        // so lets fix that here:
        for wheel in self.wheels.iter_mut() { 
            wheel.rotate(-direction, car_scene_context);
        }
    }

    pub fn update(&mut self, car_scene_context: &mut CarSceneContext) {
        if car_scene_context.keyboard.get_keystate(Keycode::Z).is_down() {
            self.rotate_wheels(1f32, car_scene_context); // ccw
        }
        if car_scene_context.keyboard.get_keystate(Keycode::X).is_down() {
            self.rotate_wheels(-1f32, car_scene_context); // clockwise
        }
    }
}