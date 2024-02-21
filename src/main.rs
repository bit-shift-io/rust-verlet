use sdl2::{event::Event, keyboard::Keycode, pixels::Color, gfx::primitives::DrawRenderer};
use cgmath::{InnerSpace, Vector2};
use rand::Rng;
use std::time::Duration;

mod sdl_system;
mod solver;
mod particle;
mod stick;

use crate::particle::Particle;
use crate::sdl_system::SdlSystem;
use crate::solver::Solver;
use crate::stick::Stick;

fn main() -> Result<(), String> {
    println!("Hello, world!");

    let mut sdl = SdlSystem::new("Rust Verlet", 1200, 800);
    let mut event_pump = sdl.sdl_context.event_pump()?;

    let mut solver: Solver = Solver::new();

    'running: loop {
        //let start = Instant::now();

        sdl.canvas.set_draw_color(Color::RGB(0, 0, 0));
        sdl.canvas.clear();
        
        sdl.canvas.set_draw_color(Color::RGB(255, 255, 255));
        
        sdl.canvas.filled_circle(600, 400, 380, Color::RGB(150, 150, 150)).unwrap();
        //sdl.canvas.circle(600, 400, 300, Color::RGB(150, 150, 150)).ok();

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
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
                        let p1 = solver.add_particle(Particle::new(pos1, radius, mass, col));
                        let p2 = solver.add_particle(Particle::new(pos2, radius, mass, col));
                        let p3 = solver.add_particle(Particle::new(pos3, radius, mass, col));
                    
                        let length = radius * 2f32;
                        solver.add_stick(Stick::new(length, p1, p2));
                        solver.add_stick(Stick::new(length, p1, p3));
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

                        let p1 = solver.add_particle(Particle::new(pos1, radius, mass, col));
                        let p2 = solver.add_particle(Particle::new(pos2, radius, mass, col));
                        let p3 = solver.add_particle(Particle::new(pos3, radius, mass, col));
                        let p4 = solver.add_particle(Particle::new(pos4, radius, mass, col));
                    
                        //solver.add_stick(Stick::new((pos1 - pos2).magnitude(), p1, p2));
                        //solver.add_stick(Stick::new((pos2 - pos3).magnitude(), p2, p3));
                        //solver.add_stick(Stick::new((pos3 - pos4).magnitude(), p3, p4));
                        solver.add_stick(Stick::new((pos4 - pos1).magnitude(), p4, p1));


                        solver.add_stick(Stick::new((pos1 - pos3).magnitude(), p1, p3));
                        solver.add_stick(Stick::new((pos2 - pos4).magnitude(), p2, p4));

                    }
                    

                },
                _ => {}
            }
        }


/* 
        for object in solver.particles.iter() {
            object.draw(&sdl.canvas);
        }
*/
        solver.draw(&mut sdl);

        solver.update(0.0167f32);


        sdl.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        /* 
        // Update
        fluid_sim.update(dt);

        // Render
        fluid_sim_renderer.draw();

        let duration = start.elapsed();
        dt = duration.as_nanos() as f32 / 1000000000.0;

        //println!("dt: {:?}", dt);

        // Time management!
        //Duration::from_millis(1000)
        //::std::thread::sleep(Duration::from_millis(SLEEP_PER_FRAME_MS));
        //::std::thread::sleep(Duration::from::new(0, 1_000_000_000u32 / 60));
        */
    }

    println!("Goodbye, world!");
    Ok(())   
}
