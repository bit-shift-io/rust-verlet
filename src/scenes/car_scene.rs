use cgmath::{InnerSpace, Vector2};
use sdl2::{event::Event, gfx::primitives::DrawRenderer, pixels::Color};
use rand::Rng;

use crate::{application::{Context, Scene}, particle::Particle, solver::Solver, stick::Stick};

pub fn create_wheel(solver: &mut Solver, origin: Vector2<f32>) {
    let mut rng = rand::thread_rng();

    let radius = 50.0f32;
    let divisions = 10;
    let particle_radius = 5.0f32;
    let particle_mass = 1.0f32;
    let col = Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255));
                    
    let mut particle_indexes: Vec<usize> = vec![];

    for i in 0..divisions {  
        let percent = i as f32 / divisions as f32;
        let radians = percent * 2f32 * std::f32::consts::PI;
        let x = f32::sin(radians);
        let y = f32::cos(radians);
        let pos = origin + Vector2::new(x * radius, y * radius);
        let p_idx = solver.add_particle(Particle::new(pos, particle_radius, particle_mass, col));
        particle_indexes.push(p_idx);      
    }

    // add opposite sticks
    let half_divisions = divisions / 2;
    for i in 0..half_divisions { 
        let opposite_division = i + half_divisions;
        let p1_idx = particle_indexes[i];
        let p2_idx = particle_indexes[opposite_division];

        let p1 = &solver.particles[p1_idx];
        let p2 = &solver.particles[p2_idx];

        solver.add_stick(Stick::new((p1.position_current - p2.position_current).magnitude(), p1_idx, p2_idx));           
    }

    // add adjacent sticks
    for i in 0..divisions {
        let p1_idx = particle_indexes[i];
        let p2_idx = if (i + 1) == divisions { particle_indexes[0] } else { particle_indexes[i + 1] };

        let p1 = &solver.particles[p1_idx];
        let p2 = &solver.particles[p2_idx];

        solver.add_stick(Stick::new((p1.position_current - p2.position_current).magnitude(), p1_idx, p2_idx));           
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
                create_wheel(&mut self.solver, origin);


            },
            _ => {}
        }
    }
}