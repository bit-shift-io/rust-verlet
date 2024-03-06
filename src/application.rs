use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode};

use crate::sdl_system::SdlSystem;

pub struct Context<'a> {
    pub sdl: &'a mut SdlSystem,
}

pub trait Scene {
    fn update(&mut self, context: &mut Context);
    fn draw(&mut self, context: &mut Context);
    fn process_event(&mut self, context: &mut Context, event: Event);
}

// https://stackoverflow.com/questions/36936221/pass-self-reference-to-contained-objects-function
// https://stackoverflow.com/questions/71890445/how-can-i-store-variables-in-traits-so-that-it-can-be-used-in-other-methods-on-t


pub struct Application<'a> {
    pub sdl: &'a mut SdlSystem,
    pub scenes: Vec<Box<dyn Scene>>,
    pub current_scene_idx: usize,
}

impl<'a> Application<'a> {
    pub fn new(sdl: &'a mut SdlSystem) -> Self {
        Self { sdl, scenes: vec![], current_scene_idx: 0 }
    }

    pub fn register_scene(&mut self, scene: Box<dyn Scene>) {
        let was_empty = self.scenes.is_empty();
        self.scenes.push(scene);
        if was_empty {
            self.current_scene_idx = 0;
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        if self.scenes.is_empty() {
            return Ok(()); // really want to return an error!
        }

        let mut event_pump = self.sdl.sdl_context.event_pump()?;

        'running: loop {
            {
                let mut context = Context{ sdl: self.sdl };
                
                // Handle events
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit {..} |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            break 'running;
                        },
                        Event::KeyDown { keycode: Some(Keycode::Backquote), .. } => {
                            // change the current scene_idx
                            self.current_scene_idx += 1;
                            if (self.current_scene_idx >= self.scenes.len()) {
                                self.current_scene_idx = 0;
                            }
                        },
                        _ => {}
                    }

                    let current_scene = &mut self.scenes[self.current_scene_idx];
                    current_scene.process_event(&mut context, event);
                }
            }

            {
                let mut context = Context{ sdl: self.sdl };
                let current_scene = &mut self.scenes[self.current_scene_idx];
                current_scene.update(&mut context);
                current_scene.draw(&mut context);
            }

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
}