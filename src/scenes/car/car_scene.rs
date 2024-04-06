use std::{cell::RefCell, rc::Rc};

use cgmath::{InnerSpace, Vector2};
use sdl2::{event::Event, gfx::primitives::DrawRenderer, pixels::Color};
use rand::Rng;

use crate::{application::{Context, Scene}, v2::particle::Particle, v2::solver::Solver, v2::stick::Stick, v2::body::Body};


pub struct CarScene {
    pub solver: Solver,
}

impl CarScene {
    pub fn new() -> Self {
        let mut solver = Solver::new();

        let ground_plane = Box::new(Body::create_line(Vector2::new(100.0f32, 800.0f32), Vector2::new(1000.0f32, 800.0f32), 20.0f32));
        solver.add_body(ground_plane);

        Self { solver }
    }
}

impl Scene for CarScene {
    fn update(&mut self, context: &mut Context) {
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
        match event {
            Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Left, x, y, .. } => {
                let xf = x as f32;
                let yf = y as f32;
                let mut rng = rand::thread_rng();

                let shape = rng.gen_range(0..=1);

                // wheel
                let origin = Vector2::new(xf, yf);
                //let body = create_wheel(origin);
                let body = Box::new(Body::create_wheel(origin));
                self.solver.add_body(body);
            },
            _ => {}
        }
    }
}