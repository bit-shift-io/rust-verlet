use sdl2::{event::Event, keyboard::Keycode};

mod sdl_system;

use crate::sdl_system::SdlSystem;

fn main() -> Result<(), String> {
    println!("Hello, world!");

    let sdl = SdlSystem::new("Rust Verlet", 800, 600);
    let mut event_pump = sdl.sdl_context.event_pump()?;

    'running: loop {
        //let start = Instant::now();

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

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
