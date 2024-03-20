use crate::sdl_system::SdlSystem;

use super::body::Body;

pub struct Solver<'a> {
    pub bodies: Vec<Box<Body<'a>>>,
}

impl<'a> Solver<'a> {
    pub fn new() -> Self {
        Self { bodies: vec![] }
    }

    pub fn add_body(&mut self, body: Box<Body<'a>>) {
        self.bodies.push(body);
    }


    pub fn update(&mut self, dt: f32) {
        const SUB_STEPS: u32 = 16;
        let sub_dt: f32 = dt / SUB_STEPS as f32;
        for _ in 0..SUB_STEPS {
            /* 
            self.apply_gravity();
            self.apply_containment_constraint();
            self.solve_collisions(sub_dt);
            self.update_positions(sub_dt);
            */
        }
    }

    pub fn draw(&self, sdl: &mut SdlSystem) {
        /* 
        for particle in self.particles.iter() {
            particle.draw(&sdl.canvas);
        }

        sdl.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for stick in self.sticks.iter() {
            let p1 = &self.particles[stick.p1];
            let p2 = &self.particles[stick.p2];

            let p1_x = p1.position_current[0].round() as i32;
            let p1_y = p1.position_current[1].round() as i32;

            let p2_x = p2.position_current[0].round() as i32;
            let p2_y = p2.position_current[1].round() as i32;

            
            let _ = sdl.canvas.draw_line(Point::new(p1_x, p1_y), Point::new(p2_x, p2_y));
        }
        */
    }
}