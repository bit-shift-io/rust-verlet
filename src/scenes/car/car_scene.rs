use std::{cell::RefCell, rc::Rc};

use cgmath::{InnerSpace, Vector2};
use sdl2::{event::Event, gfx::primitives::DrawRenderer, keyboard::Keycode, pixels::Color};
use rand::Rng;

use crate::{application::{Context, Scene}, keyboard::Keyboard, mouse::Mouse, v2::{body::Body, particle::Particle, solver::Solver, stick::Stick}};


pub struct CarScene {
    pub solver: Solver,

    pub wheel_1: Rc<RefCell<Body>>,

    pub wheel_2_surface_handle: Rc<RefCell<Body>>,
    pub wheel_2_interior_handle: Rc<RefCell<Body>>,

    pub keyboard: Keyboard,
    pub mouse: Mouse,
}

impl CarScene {
    pub fn new() -> Self {
        let mut solver = Solver::new();

        let ground_plane = Rc::new(RefCell::new(Body::create_line(Vector2::new(100.0f32, 800.0f32), Vector2::new(600.0f32, 800.0f32), 8.0f32)));
        ground_plane.borrow_mut().set_static(true);
        solver.add_body(&ground_plane);

        let ground_plane_2 = Rc::new(RefCell::new(Body::create_line(Vector2::new(600.0f32, 800.0f32), Vector2::new(1000.0f32, 700.0f32), 8.0f32)));
        ground_plane_2.borrow_mut().set_static(true);
        solver.add_body(&ground_plane_2);

        let wheel_1 = Rc::new(RefCell::new(Body::create_stick_spoke_wheel(Vector2::new(300.0f32, 200.0f32))));
        //wheel.as_ref().borrow_mut().set_gravity_enabled(false); // to let us test rotational force
        solver.add_body(&wheel_1);

        let (wheel_2_surface, wheel_2_interior) = Body::create_fluid_filled_wheel(Vector2::new(400.0f32, 200.0f32));
  
        let wheel_2_surface_handle = Rc::new(RefCell::new(wheel_2_surface));
        let wheel_2_interior_handle = Rc::new(RefCell::new(wheel_2_interior));

        //wheel.as_ref().borrow_mut().set_gravity_enabled(false); // to let us test rotational force
        solver.add_body(&wheel_2_surface_handle);
        solver.add_body(&wheel_2_interior_handle);

        Self { 
            solver, 
            wheel_1,
            wheel_2_surface_handle,
            wheel_2_interior_handle,
            keyboard: Keyboard::new(),
            mouse: Mouse::new()
        }
    }
}

impl CarScene {
    fn rotate_wheel_wheel(&self, direction: f32, wheel: &Rc<RefCell<Body>>) {
        // todo: get the Body center to rotate around
        // todo: we should add Body.Axis class to handle this automatically for us
        // todo: unit test add_rotational_force_around_point
        let opposite_particle_idx = (wheel.borrow().particles.len() as f32 / 2f32) as usize;
        let p0 = wheel.borrow().particles[0].borrow().pos;
        let p1 = wheel.borrow().particles[opposite_particle_idx].borrow().pos;
        let centre = p0 + (p1 - p0) * 0.5f32;
        let force_magnitude = 100f32;
        wheel.borrow_mut().add_rotational_force_around_point(centre, force_magnitude * direction);
    }

    fn rotate_wheel(&mut self, direction: f32) {
        // something wrong here, z should be counterclockwise. x should be clockwise
        // be it seems reversed!? because in SDL the y-axis is mirrored around the x-axis
        // so lets fix that here:
        self.rotate_wheel_wheel(-direction, &self.wheel_1);
        self.rotate_wheel_wheel(-direction, &self.wheel_2_surface_handle);
    }
}

impl Scene for CarScene {
    fn update(&mut self, context: &mut Context) {
        self.keyboard.update();
        self.mouse.update();

        self.wheel_1.borrow_mut().zero_forces();
        self.wheel_1.borrow_mut().add_gravity();

        self.wheel_2_surface_handle.borrow_mut().zero_forces();
        self.wheel_2_surface_handle.borrow_mut().add_gravity();

        if self.keyboard.get_keystate(Keycode::Z).is_down() {
            self.rotate_wheel(1f32); // ccw
        }
        if self.keyboard.get_keystate(Keycode::X).is_down() {
            self.rotate_wheel(-1f32); // clockwise
        }


        self.solver.update(0.0167f32);
    }

    fn draw(&mut self, context: &mut Context) {
        context.sdl.canvas.set_draw_color(Color::RGB(128, 255, 255));
        context.sdl.canvas.clear();

        self.solver.draw(context.sdl);

        context.sdl.canvas.present();

/* 
        context.sdl.canvas.set_draw_color(Color::RGB(0, 0, 0));
        context.sdl.canvas.clear();
        context.sdl.canvas.set_draw_color(Color::RGB(255, 255, 255));
        context.sdl.canvas.filled_circle(600, 400, 380, Color::RGB(150, 150, 150)).unwrap();

        self.solver.as_mut().draw(context.sdl);

        context.sdl.canvas.present();
        */
    }

    fn process_event(&mut self, context: &mut Context, event: Event) {
        self.mouse.process_event(event.clone());
        self.keyboard.process_event(event.clone());
/* 
        match event {
            Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Left, x, y, .. } => {
                let xf = x as f32;
                let yf = y as f32;
                let mut rng = rand::thread_rng();

                let shape = rng.gen_range(0..=1);

                // wheel
                let origin = Vector2::new(xf, yf);
                //let body = create_stick_spoke_wheel(origin);
                let body = Rc::new(RefCell::new(Body::create_stick_spoke_wheel(origin)));
                self.solver.add_body(&body);
            },

            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                self.rotate_wheel(1f32);
            },

            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                self.rotate_wheel(-1f32);
            },
            _ => {}
        }
        */
    }
}