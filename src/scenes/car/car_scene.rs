use std::{cell::RefCell, rc::Rc};

use cgmath::{InnerSpace, Vector2};
use sdl2::{event::Event, gfx::primitives::DrawRenderer, pixels::Color};
use rand::Rng;

use crate::{application::{Context, Scene}, v2::particle::Particle, v2::solver::Solver, v2::stick::Stick, v2::body::Body};

impl Body {
    pub fn create_wheel(origin: Vector2<f32>) -> Self {
        let mut rng = rand::thread_rng();

        let radius = 20.0f32;
        let divisions = 10;
        let particle_radius = 5.0f32;
        let particle_mass = 1.0f32;
        let col = Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255));
                        
        let mut particle_indexes: Vec<usize> = vec![];
    
        let mut body = Body::new();
    
        for i in 0..divisions {  
            let percent = i as f32 / divisions as f32;
            let radians = percent * 2f32 * std::f32::consts::PI;
            let x = f32::sin(radians);
            let y = f32::cos(radians);
            let pos = origin + Vector2::new(x * radius, y * radius);
    
            let particle = Rc::new(RefCell::new(Particle::new(pos, particle_radius, particle_mass, col)));
            body.add_particle(particle);     
        }

        // add opposite sticks
        let half_divisions = divisions / 2;
        for i in 0..half_divisions { 
            let opposite_division = i + half_divisions;

            let stick = {
                let p1 = Rc::clone(&body.particles[i]);
                let p2 = Rc::clone(&body.particles[opposite_division]);

                Rc::new(RefCell::new(Stick::new(p1, p2)))
            };

            //let mut stick = Box::new(Stick::new(p1, p2));     
            body.add_stick(stick);
            /* ERROR FOR ABOVE LINE:
            cannot borrow `body` as mutable because it is also borrowed as immutable
            mutable borrow occurs hererustcClick for full compiler diagnostic
            car_scene.rs(38, 27): immutable borrow occurs here
            car_scene.rs(7, 6): lifetime `'a` defined here
            car_scene.rs(57, 9): returning this value requires that `body.particles` is borrowed for `'a`
            */
        }

        // add adjacent sticks
        for i in 0..divisions {
            let p1 = Rc::clone(&body.particles[i]);
            let p2 = if (i + 1) == divisions { Rc::clone(&body.particles[0]) } else { Rc::clone(&body.particles[i + 1]) };
            
            let stick = Rc::new(RefCell::new(Stick::new(p1, p2)));
            body.add_stick(stick);          
        }

        body
    }
}


pub struct CarScene {
    pub solver: Solver,
}

impl CarScene {
    pub fn new() -> Self {
        let solver = Solver::new(); //Box::new(Solver::new());
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