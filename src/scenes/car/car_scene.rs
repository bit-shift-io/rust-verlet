use std::{cell::RefCell, rc::Rc};

use cgmath::{InnerSpace, Vector2};
use sdl2::{event::Event, gfx::primitives::DrawRenderer, keyboard::Keycode, pixels::Color};
use rand::Rng;

use crate::{application::{Context, Scene}, keyboard::Keyboard, mouse::Mouse, v2::{attachment::Attachment, body::Body, particle::Particle, solver::Solver, stick::Stick}};

use super::car::Car;

pub struct CarSceneContext<'a> {
    pub keyboard: &'a mut Keyboard,
    pub mouse: &'a mut Mouse,
}

pub struct CarScene {
    pub solver: Solver,
    pub car: Car,
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

        let car = Car::new();
        car.add_to_solver(&mut solver);

        Self { 
            solver, 
            car,
            keyboard: Keyboard::new(),
            mouse: Mouse::new()
        }
    }
}

impl Scene for CarScene {
    fn update(&mut self, context: &mut Context) {
        //println!("car scnee update start");

        self.keyboard.update();
        self.mouse.update();

        let mut car_scene_context = CarSceneContext{ keyboard: &mut self.keyboard, mouse: &mut self.mouse };
        self.car.update(&mut car_scene_context);

        self.solver.update(0.0167f32);
        //println!("car scnee update end");
    }

    fn draw(&mut self, context: &mut Context) {
        context.sdl.canvas.set_draw_color(Color::RGB(128, 255, 255));
        context.sdl.canvas.clear();
        self.solver.draw(context.sdl);
        context.sdl.canvas.present();
    }

    fn process_event(&mut self, context: &mut Context, event: Event) {
        self.mouse.process_event(event.clone());
        self.keyboard.process_event(event.clone());
    }
}