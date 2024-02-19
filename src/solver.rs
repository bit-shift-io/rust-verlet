use sdl2::pixels::Color;
use cgmath::{InnerSpace, Vector2};

use crate::verlet_object::VerletObject;
use crate::stick_constraint::StickConstraint;

pub struct Solver {
    pub gravity: Vector2<f32>,
    pub objects: Vec<VerletObject>,
    pub stick_constraints: Vec<StickConstraint>
}

impl Solver {
    pub fn add_object(&mut self, x: f32, y: f32, radius: i16, color: Color) -> usize {
        let handle = self.objects.len();
        self.objects.push(VerletObject { position_current: Vector2::new(x, y), position_old: Vector2::new(x, y), acceleration: Vector2::new(0f32, 0f32), radius: radius, color: color });
        return handle;
    }

    pub fn add_stick_constraint(&mut self, p1: usize, p2: usize, length: f32) -> usize {
        let handle = self.stick_constraints.len();
        self.stick_constraints.push(StickConstraint { p1: p1, p2: p2, length: length });
        return handle;
    }

    pub fn update(&mut self, dt: f32) {
        const SUB_STEPS: u32 = 16;
        let sub_dt: f32 = dt / SUB_STEPS as f32;
        for _ in 0..SUB_STEPS {
            self.apply_gravity();
            self.apply_constraint();
            self.solve_collisions();
            self.update_positions(sub_dt);
            self.update_stick_constraints(sub_dt);
        }
    }

    pub fn update_stick_constraints(&mut self, dt: f32) {
        for stick in self.stick_constraints.iter_mut() {
            let p1 = &self.objects[stick.p1];
            let p2 = &self.objects[stick.p2];

            let difference = p1.position_current - p2.position_current;
            let diff_length = difference.magnitude();
            let diff_factor = (stick.length - diff_length) / diff_length * 0.5;
            let offset = difference * diff_factor;
    
            {
                let p1mut = &mut self.objects[stick.p1];
                p1mut.position_current += offset;
            }

            {
                let p2mut = &mut self.objects[stick.p2];
                p2mut.position_current -= offset;
            }
        }
    }

    pub fn update_positions(&mut self, dt: f32) {
        for obj in self.objects.iter_mut() {
            obj.update_position(dt);
        }
    }
    
    pub fn apply_gravity(&mut self) {
        for obj in self.objects.iter_mut() {
            obj.accelerate(self.gravity);
        }
    }

    pub fn apply_constraint(&mut self) {
        let position: Vector2<f32> = Vector2::new(600f32, 400f32);
        let radius: f32 = 300f32;
        for obj in self.objects.iter_mut() {
            let to_obj: Vector2<f32> = obj.position_current - position;
            let dist: f32 = (to_obj[0].powf(2f32) + to_obj[1].powf(2f32)).sqrt();

            if dist > radius - obj.radius as f32 {
                let n: Vector2<f32> = to_obj / dist;
                obj.position_current = position + n * (radius - obj.radius as f32);
            }
        }
    }

    pub fn solve_collisions(&mut self) {
        let object_count: &usize = &self.objects.len();
        for i in 0..*object_count {
            for k in (&i+1)..*object_count {
                let collision_axis: Vector2<f32> = self.objects[i].position_current - self.objects[k].position_current;
                let dist: f32 = (collision_axis[0].powf(2f32) + collision_axis[1].powf(2f32)).sqrt();
                let min_dist: i16 = self.objects[i].radius + self.objects[k].radius;
                if dist < min_dist as f32 {
                    let n: Vector2<f32> = collision_axis / dist;
                    let delta: f32 = min_dist as f32 - dist;
                    self.objects[i].position_current += 0.5f32 * delta * n;
                    self.objects[k].position_current -= 0.5f32 * delta * n;
                }
            }
        }
    }
}
