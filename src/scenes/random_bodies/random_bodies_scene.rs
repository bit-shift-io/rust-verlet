use cgmath::{InnerSpace, Vector2};
use sdl2::{event::Event, gfx::primitives::DrawRenderer, pixels::Color};
use rand::Rng;

use crate::{application::{Context, Scene}, v1::particle::Particle, v1::solver::Solver, v1::stick::Stick};


pub struct RandomBodiesScene {
    pub solver: Box<Solver>,
}

impl RandomBodiesScene {
    pub fn new() -> Self {
        let solver = Box::new(Solver::new());
        Self { solver }
    }
}

impl Scene for RandomBodiesScene {
    fn update(&mut self, context: &mut Context) {
        self.solver.as_mut().update(0.0167f32);
    }

    fn draw(&mut self, context: &mut Context) {
        context.sdl.canvas.set_draw_color(Color::RGB(0, 0, 0));
        context.sdl.canvas.clear();
        context.sdl.canvas.set_draw_color(Color::RGB(255, 255, 255));
        context.sdl.canvas.filled_circle(600, 400, 380, Color::RGB(150, 150, 150)).unwrap();

        self.solver.as_mut().draw(context.sdl);

        context.sdl.canvas.present();
    }

    fn process_event(&mut self, context: &mut Context, event: Event) {
        match event {
            Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Left, x, y, .. } => {
                let xf = x as f32;
                let yf = y as f32;
                let mut rng = rand::thread_rng();

                let shape = rng.gen_range(0..=1);

                // chain of 3 circles
                if shape == 0 {
                    let radius = rng.gen_range(5..50) as f32;
                    let pos1 = Vector2::new(xf, yf);
                    let pos2 = Vector2::new(xf + radius, yf);
                    let pos3 = Vector2::new(xf - radius, yf);
                    let col = Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255));
                    let mass = radius;
                    let p1 = self.solver.add_particle(Particle::new(pos1, radius, mass, col));
                    let p2 = self.solver.add_particle(Particle::new(pos2, radius, mass, col));
                    let p3 = self.solver.add_particle(Particle::new(pos3, radius, mass, col));
                
                    let length = radius * 2f32;
                    self.solver.add_stick(Stick::new(length, p1, p2));
                    self.solver.add_stick(Stick::new(length, p1, p3));
                }

                // box
                if shape == 1 {
                    let radius = rng.gen_range(5..50) as f32;

                    let pos1 = Vector2::new(xf - radius, yf - radius);
                    let pos2 = Vector2::new(xf + radius, yf - radius);
                    let pos3 = Vector2::new(xf + radius, yf + radius);
                    let pos4 = Vector2::new(xf - radius, yf + radius);

                    let col = Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255));
                    let mass = radius;

                    let p1 = self.solver.add_particle(Particle::new(pos1, radius, mass, col));
                    let p2 = self.solver.add_particle(Particle::new(pos2, radius, mass, col));
                    let p3 = self.solver.add_particle(Particle::new(pos3, radius, mass, col));
                    let p4 = self.solver.add_particle(Particle::new(pos4, radius, mass, col));
                
                    //solver.add_stick(Stick::new((pos1 - pos2).magnitude(), p1, p2));
                    //solver.add_stick(Stick::new((pos2 - pos3).magnitude(), p2, p3));
                    //solver.add_stick(Stick::new((pos3 - pos4).magnitude(), p3, p4));
                    self.solver.add_stick(Stick::new((pos4 - pos1).magnitude(), p4, p1));


                    self.solver.add_stick(Stick::new((pos1 - pos3).magnitude(), p1, p3));
                    self.solver.add_stick(Stick::new((pos2 - pos4).magnitude(), p2, p4));
                }
            },
            _ => {}
        }
    }
}