use std::{cell::RefCell, rc::Rc};

use cgmath::{InnerSpace, Vector2};
use sdl2::{event::Event, gfx::primitives::DrawRenderer, keyboard::Keycode, pixels::Color};
use rand::Rng;

use crate::{application::{Context, Scene}, keyboard::Keyboard, mouse::Mouse, v2::{attachment::Attachment, body::Body, particle::Particle, position::Position, solver::Solver, stick::Stick}};

use super::car_scene::{self, CarSceneContext};


pub struct Car {
    pub wheel_1_surface_handle: Rc<RefCell<Body>>,
    pub wheel_1_interior_handle: Rc<RefCell<Body>>,
    pub wheel_1_axle: Rc<RefCell<Attachment>>,

    pub wheel_2_surface_handle: Rc<RefCell<Body>>,
    pub wheel_2_interior_handle: Rc<RefCell<Body>>,
    pub wheel_2_axle: Rc<RefCell<Attachment>>,

    pub chassis: Rc<RefCell<Body>>,
}

impl Car {
    pub fn new() -> Self {
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
        }
    }

    pub fn add_to_solver(&self, solver: &mut Solver) {
        solver.add_body(&self.wheel_1_surface_handle);
        solver.add_body(&self.wheel_1_interior_handle);

        solver.add_body(&self.wheel_2_surface_handle);
        solver.add_body(&self.wheel_2_interior_handle);

        solver.add_body(&self.chassis);
    }

    fn create_and_attach_wheel_axle(wheel: &Rc<RefCell<Body>>) -> Rc<RefCell<Attachment>> {
        // lets make an axle at the centre of the wheel
        let axle = Rc::new(RefCell::new(Attachment::new()));
        axle.as_ref().borrow_mut().add_even_distribution_of_incoming_particles(&wheel.as_ref().borrow().particles, 1f32, 4);
        axle.as_ref().borrow_mut().add_outgoing_particles(&wheel.as_ref().borrow().particles, 1f32);
        axle.as_ref().borrow_mut().update(0f32);
        wheel.as_ref().borrow_mut().add_attachment(&axle);
        axle
    }

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
    }

    fn rotate_wheel(&mut self, direction: f32) {
        // something wrong here, z should be counterclockwise. x should be clockwise
        // be it seems reversed!? because in SDL the y-axis is mirrored around the x-axis
        // so lets fix that here:
        self.rotate_wheel_wheel(-direction, &self.wheel_1_surface_handle);
        self.rotate_wheel_wheel(-direction, &self.wheel_2_surface_handle);
    }

    pub fn update(&mut self, car_scene_context: &mut CarSceneContext) {
        self.wheel_1_interior_handle.borrow_mut().zero_forces();
        self.wheel_1_interior_handle.borrow_mut().add_gravity();
        self.wheel_1_surface_handle.borrow_mut().zero_forces();
        self.wheel_1_surface_handle.borrow_mut().add_gravity();

        self.wheel_2_interior_handle.borrow_mut().zero_forces();
        self.wheel_2_interior_handle.borrow_mut().add_gravity();
        self.wheel_2_surface_handle.borrow_mut().zero_forces();
        self.wheel_2_surface_handle.borrow_mut().add_gravity();

        if car_scene_context.keyboard.get_keystate(Keycode::Z).is_down() {
            self.rotate_wheel(1f32); // ccw
        }
        if car_scene_context.keyboard.get_keystate(Keycode::X).is_down() {
            self.rotate_wheel(-1f32); // clockwise
        }
    }
}