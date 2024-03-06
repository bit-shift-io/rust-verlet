use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode};

use crate::sdl_system::SdlSystem;

pub trait Scene {
    fn update(&self, application: Application);
    fn render(&self, application: Application);
    fn process_event(&self, application: Application, event: Event);
}

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
        let mut event_pump = self.sdl.sdl_context.event_pump()?;

        'running: loop {
            // Handle events
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    },
                    _ => {}
                }

                self.process_event(event);
            }

            self.update();
            self.draw();

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }

    pub fn process_event(&mut self, event: Event) {
        println!("prcoess_event");
    }

    pub fn update(&mut self) {
        println!("update");
    }

    pub fn draw(&mut self) {
        println!("draw");
    }
}