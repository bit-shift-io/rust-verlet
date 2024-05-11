use std::collections::HashMap;

use sdl2::{event::Event, keyboard::Keycode};

pub struct Keystate {
    pub is_down: bool,
    pub was_down: bool,
}

impl Keystate {
    pub fn new() -> Self {
        Self { is_down: false, was_down: false }
    }

    pub fn update(&mut self) {
        self.was_down = self.is_down;
    }

    pub fn is_down(&self) -> bool {
        return self.is_down;
    }
}

pub struct Keyboard {
    keystates: HashMap<Keycode, Keystate>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self { keystates: HashMap::new() }
    }

    pub fn get_keystate(&mut self, keycode: Keycode) -> &Keystate {
        // https://stackoverflow.com/questions/73801225/hashmap-get-or-insert-and-return-a-reference-from-a-function
        self.keystates.entry(keycode).or_insert_with(|| Keystate::new().into())
    }

    fn get_keystate_mut(&mut self, keycode: Keycode) -> &mut Keystate {
        // https://stackoverflow.com/questions/73801225/hashmap-get-or-insert-and-return-a-reference-from-a-function
        self.keystates.entry(keycode).or_insert_with(|| Keystate::new().into())
    }

    pub fn process_event(&mut self, event: Event) {
        match event {
            Event::KeyDown { keycode, .. } => {
                if let Some(kc) = keycode {
                    let keystate = self.get_keystate_mut(kc);
                    keystate.is_down = true;
                }
            },

            Event::KeyUp { keycode, .. } => {
                if let Some(kc) = keycode {
                    let keystate = self.get_keystate_mut(kc);
                    keystate.is_down = false;
                }
            },

            _ => {}
        }
    }

    pub fn update(&mut self) {
        for (_, keystate) in self.keystates.iter_mut() {
            keystate.update();
        }
    }
}