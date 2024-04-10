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
            body.add_particle(&particle);
        }

        body
    }

    pub fn create_stick_spoke_wheel(origin: Vector2<f32>) -> Self {
        let mut rng = rand::thread_rng();

        let radius = 20.0f32;
        let divisions = 8;
        let particle_radius = 8.0f32;
        let particle_mass = 1.0f32;
        let col = Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255));
                
        let mut body = Body::new();
    
        for i in 0..divisions {  
            let percent = i as f32 / divisions as f32;
            let radians = percent * 2f32 * std::f32::consts::PI;
            let x = f32::sin(radians);
            let y = f32::cos(radians);
            let pos = origin + Vector2::new(x * radius, y * radius);
    
            let particle = Rc::new(RefCell::new(Particle::new(pos, particle_radius, particle_mass, col)));
            body.add_particle(&particle);     
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
   
            body.add_stick(&stick);
        }

        // add adjacent sticks
        for i in 0..divisions {
            let p1 = Rc::clone(&body.particles[i]);
            let p2 = if (i + 1) == divisions { Rc::clone(&body.particles[0]) } else { Rc::clone(&body.particles[i + 1]) };
            
            let stick = Rc::new(RefCell::new(Stick::new(p1, p2)));
            body.add_stick(&stick);          
        }

        body
    }


    pub fn create_fluid_filled_wheel(origin: Vector2<f32>) -> (Self, Self) {
        let mut rng = rand::thread_rng();

        // create the surface (tyre) body    
        let mut surface_body = Body::new();
        {
            let radius = 20.0f32;
            let divisions = 8;
            let particle_radius = 8.0f32;
            let particle_mass = 1.0f32;
            let col = Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255));
               
            
            for i in 0..divisions {  
                let percent = i as f32 / divisions as f32;
                let radians = percent * 2f32 * std::f32::consts::PI;
                let x = f32::sin(radians);
                let y = f32::cos(radians);
                let pos = origin + Vector2::new(x * radius, y * radius);
        
                let particle = Rc::new(RefCell::new(Particle::new(pos, particle_radius, particle_mass, col)));
                surface_body.add_particle(&particle);     
            }

            // add adjacent sticks
            for i in 0..divisions {
                let p1 = Rc::clone(&surface_body.particles[i]);
                let p2 = if (i + 1) == divisions { Rc::clone(&surface_body.particles[0]) } else { Rc::clone(&surface_body.particles[i + 1]) };
                
                let stick = Rc::new(RefCell::new(Stick::new(p1, p2)));
                surface_body.add_stick(&stick);          
            }
        }

        // create the interior (fluid) body
        let mut interior_body = Body::new();
        {
            let radius = 10.0f32;
            let divisions = 6;
            let particle_radius = 4.0f32;
            let particle_mass = 1.0f32;
            let col = Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255));
               
            interior_body.set_collides_with_self(true);

            for i in 0..divisions {  
                let percent = i as f32 / divisions as f32;
                let radians = percent * 2f32 * std::f32::consts::PI;
                let x = f32::sin(radians);
                let y = f32::cos(radians);
                let pos = origin + Vector2::new(x * radius, y * radius);
        
                let particle = Rc::new(RefCell::new(Particle::new(pos, particle_radius, particle_mass, col)));
                interior_body.add_particle(&particle);     
            }
        }

        (surface_body, interior_body)
    }
}
