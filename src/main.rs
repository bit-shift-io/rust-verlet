use sdl2::{event::Event, keyboard::Keycode, pixels::Color, gfx::primitives::DrawRenderer};
use cgmath::Vector2;
use rand::Rng;
use std::time::Duration;

mod sdl_system;
mod solver;
mod verlet_object;

use crate::sdl_system::SdlSystem;
use crate::solver::Solver;

fn main() -> Result<(), String> {
    println!("Hello, world!");

    let mut sdl = SdlSystem::new("Rust Verlet", 1200, 800);
    let mut event_pump = sdl.sdl_context.event_pump()?;


    let mut solver: Solver = Solver { gravity: Vector2::new(0f32, 1000f32), objects: vec![]};

    'running: loop {
        //let start = Instant::now();

        sdl.canvas.set_draw_color(Color::RGB(0, 0, 0));
        sdl.canvas.clear();
        
        sdl.canvas.set_draw_color(Color::RGB(255, 255, 255));
        
        sdl.canvas.filled_circle(600, 400, 300, Color::RGB(150, 150, 150)).unwrap();
        //sdl.canvas.circle(600, 400, 300, Color::RGB(150, 150, 150)).ok();

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Left, x, y, .. } => {
                    let mut rng = rand::thread_rng();
                    solver.add_object(x as f32, y as f32, rng.gen_range(5..50), Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255)));
                },
                _ => {}
            }
        }



        for object in solver.objects.iter() {
            object.draw(&sdl.canvas);
        }

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
