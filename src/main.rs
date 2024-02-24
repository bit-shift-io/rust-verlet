use cloth::CMouse;
use sdl2::mouse::MouseWheelDirection;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, gfx::primitives::DrawRenderer};
use cgmath::{InnerSpace, Vector2};
use rand::Rng;
use std::time::Duration;

mod sdl_system;
mod solver;
mod particle;
mod stick;
mod cloth;

use crate::particle::Particle;
use crate::sdl_system::SdlSystem;
use crate::solver::Solver;
use crate::stick::Stick;
use crate::cloth::Cloth;


fn scene_random_bodies(sdl: &mut SdlSystem) -> Result<(), String> {
    let mut event_pump = sdl.sdl_context.event_pump()?;
    let mut solver: Solver = Solver::new();

    'running: loop {
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

        solver.update(0.0167f32);

        sdl.canvas.set_draw_color(Color::RGB(0, 0, 0));
        sdl.canvas.clear();
        sdl.canvas.set_draw_color(Color::RGB(255, 255, 255));
        sdl.canvas.filled_circle(600, 400, 380, Color::RGB(150, 150, 150)).unwrap();

        solver.draw(sdl);

        sdl.canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}



fn scene_cloth(sdl: &mut SdlSystem) -> Result<(), String> {
    let mut event_pump = sdl.sdl_context.event_pump()?;
    let mut cloth: Cloth = Cloth::new(20, 20, 20, 100, 100);
    let mut mouse = CMouse::new();
    
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    mouse.update_position(x, y);

                    if !mouse.left_button_down && mouse_btn == sdl2::mouse::MouseButton::Left {
                        mouse.left_button_down = true;
                    }

                    if !mouse.right_button_down && mouse_btn == sdl2::mouse::MouseButton::Right {
                        mouse.right_button_down = true;
                    }
                },
                Event::MouseButtonUp { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                    if mouse.left_button_down && mouse_btn == sdl2::mouse::MouseButton::Left {
                        mouse.left_button_down = false;
                    }
                    if mouse.right_button_down && mouse_btn == sdl2::mouse::MouseButton::Right {
                        mouse.right_button_down = false;
                    }
                },
                Event::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel } => {
                    mouse.update_position(x, y);
                },
                Event::MouseWheel { timestamp, window_id, which, x, y, direction, precise_x, precise_y } => {
                    if direction == MouseWheelDirection::Normal {
                        mouse.increase_cursor_size(10f32);
                    }
                    if direction == MouseWheelDirection::Flipped {
                        mouse.increase_cursor_size(-10f32);
                    }
                },
                _ => {}
            }
        }

        cloth.update(0.0167f32, &mouse);

        sdl.canvas.set_draw_color(Color::RGB(255, 255, 255));
        sdl.canvas.clear();

        cloth.draw(sdl);

        sdl.canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}


fn main() -> Result<(), String> {
    let mut sdl = SdlSystem::new("Rust Verlet", 1200, 800);
    //let r = scene_random_bodies(&mut sdl);
    let r = scene_cloth(&mut sdl);
    r
}
