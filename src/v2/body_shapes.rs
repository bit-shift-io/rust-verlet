use std::{cell::RefCell, rc::Rc};

use cgmath::{InnerSpace, Vector2};
use sdl2::{pixels::Color};
use rand::Rng;

use crate::{application::{Context, Scene}, v2::particle::Particle, v2::stick::Stick, v2::body::Body};


impl Body {
    pub fn create_line(p1: Vector2<f32>, p2: Vector2<f32>, radius: f32) -> Self {
        let mut rng = rand::thread_rng();
        let mut body = Body::new();

        let particle_mass = 1.0f32;

        let dist = (p2 - p1).magnitude();
        let divisions = (dist / (radius * 2.0f32)) as usize;
        let delta = (p2 - p1);

        let col = Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255));
          
        for i in 0..divisions { 
            let percent = i as f32 / divisions as f32;
            let pos = p1 + (delta * percent);

            let particle = Rc::new(RefCell::new(Particle::new(pos, radius, particle_mass, col)));
            body.add_particle(particle);
        }

        body
    }

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
