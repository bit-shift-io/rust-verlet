use std::{cell::RefCell, rc::Rc};

use cgmath::{InnerSpace, Vector2};
use sdl2::{event::Event, gfx::primitives::DrawRenderer, keyboard::Keycode, pixels::Color};
use rand::Rng;

use crate::{application::{Context, Scene}, keyboard::Keyboard, mouse::Mouse, v2::{attachment::Attachment, body::Body, particle::Particle, position::Position, solver::Solver, stick::Stick}, v3::{particle_accelerator::{ParticleAccelerator, ParticleHandle, WeightedParticle}, shape_builder::ShapeBuilder, types::Vec2}};

use super::car_scene::{self, CarSceneContext};

pub struct CarWheel {
    hub_particle_handle: ParticleHandle,
    surface_particle_handles: Vec<ParticleHandle>,
    interior_particle_handles: Vec<ParticleHandle>,
}

impl CarWheel {
    pub fn new(origin: Vec2, particle_accelerator: &mut ParticleAccelerator) -> Self {
        let mask = 0x1;

        // wheel hub
        let hub_particle_handle = {
            let particle_radius = 4.0;
            let mut builder = ShapeBuilder::new();
            builder.add_particle(origin, particle_radius)
                .create_in_particle_accelerator(particle_accelerator, mask);
            builder.particle_handles.first().unwrap().clone()
        };


        // wheel surface
        let surface_particle_handles = {
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

        // to optimise this we really only need maybe 4 points to determine the centre of the wheel
        let mut weighted_particles = vec![];
        for particle_handle in surface_particle_handles.iter() {
            weighted_particles.push(WeightedParticle::new(particle_handle.clone(), 1.0));
        }
        particle_accelerator.create_weighted_average_constraint(weighted_particles, hub_particle_handle.clone());
        

        Self {
            hub_particle_handle,
            surface_particle_handles,
            interior_particle_handles
        }
    }

    fn rotate(&mut self, direction: f32) {
        /* 
        // something wrong here, z should be counterclockwise. x should be clockwise
        // be it seems reversed!? because in SDL the y-axis is mirrored around the x-axis
        // so lets fix that here:
        self.rotate_wheel_wheel(-direction, &self.wheel_1_surface_handle);
        self.rotate_wheel_wheel(-direction, &self.wheel_2_surface_handle);
        */
        /* 
        // todo: get the Body center to rotate around
        // todo: we should add Body.Axis class to handle this automatically for us
        // todo: unit test add_rotational_force_around_point
        let opposite_particle_idx = (wheel.borrow().particles.len() as f32 / 2f32) as usize;
        let p0 = wheel.borrow().particles[0].borrow().pos;
        let p1 = wheel.borrow().particles[opposite_particle_idx].borrow().pos;
        let centre = p0 + (p1 - p0) * 0.5f32;
        let force_magnitude = 50f32;
        wheel.borrow_mut().add_rotational_force_around_point(centre, force_magnitude * direction);
        */
    }
}

pub struct Car {
    pub wheels: [CarWheel; 2],
    /* 
    pub wheel_1_surface_handle: Rc<RefCell<Body>>,
    pub wheel_1_interior_handle: Rc<RefCell<Body>>,
    pub wheel_1_axle: Rc<RefCell<Attachment>>,

    pub wheel_2_surface_handle: Rc<RefCell<Body>>,
    pub wheel_2_interior_handle: Rc<RefCell<Body>>,
    pub wheel_2_axle: Rc<RefCell<Attachment>>,

    pub chassis: Rc<RefCell<Body>>,
    */
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
/* 
        let mask = 0x1;

        // wheel surface
        {
            let divisions = 8;
            let circle_radius = 20.0;
            let particle_radius = 7.0;
            ShapeBuilder::new()
                .add_adjacent_stick_circle( Vec2::new(300.0f32, 300.0f32), circle_radius, particle_radius, divisions)
                .create_in_particle_accelerator(particle_accelerator, mask);
        }

        // wheel interior
        {
            let divisions = 6;
            let circle_radius = 10.0;
            let particle_radius = 4.0;
            ShapeBuilder::new()
                .add_circle( Vec2::new(300.0f32, 300.0f32), circle_radius, particle_radius, divisions)
                .create_in_particle_accelerator(particle_accelerator, mask);
        }
*/
/* 
        let (wheel_1_surface, wheel_1_interior) = Body::create_fluid_filled_wheel(Vector2::new(300.0f32, 300.0f32));
        let wheel_1_surface_handle = Rc::new(RefCell::new(wheel_1_surface));
        let wheel_1_interior_handle = Rc::new(RefCell::new(wheel_1_interior));

        // lets make an axle at the centre of the wheel
        let wheel_1_axle = Self::create_and_attach_wheel_axle(&wheel_1_surface_handle);
        
        let (wheel_2_surface, wheel_2_interior) = Body::create_fluid_filled_wheel(Vector2::new(400.0f32, 300.0f32));
        let wheel_2_surface_handle = Rc::new(RefCell::new(wheel_2_surface));
        let wheel_2_interior_handle = Rc::new(RefCell::new(wheel_2_interior));

        // lets make an axle at the centre of the wheel
        let wheel_2_axle = Self::create_and_attach_wheel_axle(&wheel_2_surface_handle);

        //wheel_2.as_ref().borrow_mut().set_gravity_enabled(false); // to let us test rotational force
       
        // okay.... I had not considered this! I'm trying to push an Attachment, which needs to push the whole body
        // so the Attachment needs to work more like a particle. 
        let mut chassis_b = Body::new();
        let wheel_1_axle_dyn: Rc<RefCell<dyn Position>> = wheel_1_axle.clone();
        let wheel_2_axle_dyn: Rc<RefCell<dyn Position>> = wheel_2_axle.clone();
        let axle_stick = Rc::new(RefCell::new(Stick::new(&wheel_1_axle_dyn, &wheel_2_axle_dyn)));
        chassis_b.add_stick(&axle_stick);
        let chassis = Rc::new(RefCell::new(chassis_b));

        Self { 
            wheel_1_surface_handle,
            wheel_1_interior_handle,
            wheel_1_axle,

            wheel_2_surface_handle,
            wheel_2_interior_handle,
            wheel_2_axle,

            chassis
        }*/

        Self {
            wheels: [wheel_1, wheel_2],
        }
    }

    /* 
    fn create_and_attach_wheel_axle(wheel: &Rc<RefCell<Body>>) -> Rc<RefCell<Attachment>> {
        // lets make an axle at the centre of the wheel
        let axle = Rc::new(RefCell::new(Attachment::new()));
        axle.as_ref().borrow_mut().add_even_distribution_of_incoming_particles(&wheel.as_ref().borrow().particles, 1f32, 4);
        axle.as_ref().borrow_mut().add_outgoing_particles(&wheel.as_ref().borrow().particles, 1f32);
        axle.as_ref().borrow_mut().update(0f32);
        wheel.as_ref().borrow_mut().add_attachment(&axle);
        axle
    }*/

    /* 
    fn rotate_wheel_wheel(&self, direction: f32, wheel: &Rc<RefCell<Body>>) {
        // todo: get the Body center to rotate around
        // todo: we should add Body.Axis class to handle this automatically for us
        // todo: unit test add_rotational_force_around_point
        let opposite_particle_idx = (wheel.borrow().particles.len() as f32 / 2f32) as usize;
        let p0 = wheel.borrow().particles[0].borrow().pos;
        let p1 = wheel.borrow().particles[opposite_particle_idx].borrow().pos;
        let centre = p0 + (p1 - p0) * 0.5f32;
        let force_magnitude = 50f32;
        wheel.borrow_mut().add_rotational_force_around_point(centre, force_magnitude * direction);
    }*/

    fn rotate_wheel(&mut self, direction: f32) {
        
        // something wrong here, z should be counterclockwise. x should be clockwise
        // be it seems reversed!? because in SDL the y-axis is mirrored around the x-axis
        // so lets fix that here:
        /* 
        self.rotate_wheel_wheel(-direction, &self.wheel_1_surface_handle);
        self.rotate_wheel_wheel(-direction, &self.wheel_2_surface_handle);
        */
        for wheel in self.wheels.iter_mut() { 
            wheel.rotate(-direction);
        }
    }

    pub fn update(&mut self, car_scene_context: &mut CarSceneContext) {
        /* 
        self.wheel_1_interior_handle.borrow_mut().zero_forces();
        self.wheel_1_interior_handle.borrow_mut().add_gravity();
        self.wheel_1_surface_handle.borrow_mut().zero_forces();
        self.wheel_1_surface_handle.borrow_mut().add_gravity();

        self.wheel_2_interior_handle.borrow_mut().zero_forces();
        self.wheel_2_interior_handle.borrow_mut().add_gravity();
        self.wheel_2_surface_handle.borrow_mut().zero_forces();
        self.wheel_2_surface_handle.borrow_mut().add_gravity();
        */

        if car_scene_context.keyboard.get_keystate(Keycode::Z).is_down() {
            self.rotate_wheel(1f32); // ccw
        }
        if car_scene_context.keyboard.get_keystate(Keycode::X).is_down() {
            self.rotate_wheel(-1f32); // clockwise
        }
    }
}