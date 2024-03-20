use cgmath::{InnerSpace, Vector2};
use sdl2::pixels::Color;
use sdl2::rect::Point;

use crate::v1::particle::Particle;
use crate::sdl_system::SdlSystem;
use crate::v1::stick::Stick;

pub struct Solver {
    pub gravity: Vector2<f32>,
    pub particles: Vec<Particle>,
    pub sticks: Vec<Stick>
}

impl Solver {
    pub fn new() -> Self {
        Self { gravity: Vector2::new(0f32, 1000f32), particles: vec![], sticks: vec![] }
    }

    pub fn add_particle(&mut self, particle: Particle) -> usize {
        let handle = self.particles.len();
        self.particles.push(particle);
        return handle;
    }

    pub fn add_stick(&mut self, stick: Stick) -> usize {
        let handle = self.sticks.len();
        self.sticks.push(stick);
        return handle;
    }

    pub fn update(&mut self, dt: f32) {
        const SUB_STEPS: u32 = 16;
        let sub_dt: f32 = dt / SUB_STEPS as f32;
        for _ in 0..SUB_STEPS {
            self.apply_gravity();
            self.apply_containment_constraint();
            self.solve_collisions(sub_dt);
            self.update_positions(sub_dt);
        }
    }

    fn update_sticks(&mut self, dt: f32) {
        for stick in self.sticks.iter_mut() {
            let p1 = &self.particles[stick.p1];
            let p2 = &self.particles[stick.p2];

            let difference = p1.position_current - p2.position_current;
            let diff_length = difference.magnitude();
            let diff_factor = (stick.length - diff_length) / diff_length * 0.5;
            let offset = difference * diff_factor;
    
            {
                let p1mut = &mut self.particles[stick.p1];
                p1mut.position_current += offset;
            }

            {
                let p2mut = &mut self.particles[stick.p2];
                p2mut.position_current -= offset;
            }
        }
    }

    fn update_positions(&mut self, dt: f32) {
        for obj in self.particles.iter_mut() {
            obj.update_position(dt);
        }
    }
    
    fn apply_gravity(&mut self) {
        for obj in self.particles.iter_mut() {
            obj.accelerate(self.gravity);
        }
    }

    fn apply_containment_constraint(&mut self) {
        let position: Vector2<f32> = Vector2::new(600f32, 400f32);
        let radius: f32 = 380f32;
        for obj in self.particles.iter_mut() {
            let to_obj: Vector2<f32> = obj.position_current - position;
            let dist: f32 = (to_obj[0].powf(2f32) + to_obj[1].powf(2f32)).sqrt();

            if dist > radius - obj.radius as f32 {
                let n: Vector2<f32> = to_obj / dist;
                obj.position_current = position + n * (radius - obj.radius as f32);
            }
        }
    }

    fn solve_collisions(&mut self, dt: f32) {
        let object_count: &usize = &self.particles.len();
        for i in 0..*object_count {
            for k in (&i+1)..*object_count {
                let collision_axis: Vector2<f32> = self.particles[i].position_current - self.particles[k].position_current;
                let dist: f32 = (collision_axis[0].powf(2f32) + collision_axis[1].powf(2f32)).sqrt();
                let min_dist: f32 = self.particles[i].radius + self.particles[k].radius;
                if dist < min_dist as f32 {
                    let n: Vector2<f32> = collision_axis / dist;
                    let delta: f32 = min_dist as f32 - dist;
                    self.particles[i].position_current += 0.5f32 * delta * n;
                    self.particles[k].position_current -= 0.5f32 * delta * n;

                    // todo: we only want to update the sticks that are connected to particle i and k
                    //self.update_sticks(dt);
                }
            }
        }

        self.update_sticks(dt);
    }

    pub fn draw(&self, sdl: &mut SdlSystem) {
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
    }
}
