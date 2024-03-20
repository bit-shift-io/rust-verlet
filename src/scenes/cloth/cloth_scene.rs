use sdl2::{event::Event, pixels::Color};

use crate::{application::{Context, Scene}, v1::cloth::{CMouse, Cloth}};

pub struct ClothScene {
    pub cloth: Box<Cloth>,
    pub mouse: Box<CMouse>,
}

impl ClothScene {
    pub fn new() -> Self {
        let cloth = Box::new(Cloth::new(20, 20, 20, 100, 100));
        let mouse = Box::new(CMouse::new());
        Self { cloth, mouse }
    }
}

impl Scene for ClothScene {
    fn update(&mut self, context: &mut Context) {
        self.cloth.as_mut().update(0.0167f32, &self.mouse);
    }

    fn draw(&mut self, context: &mut Context) {
        context.sdl.canvas.set_draw_color(Color::RGB(255, 255, 255));
        context.sdl.canvas.clear();

        self.cloth.as_mut().draw(context.sdl);

        context.sdl.canvas.present();
    }

    fn process_event(&mut self, context: &mut Context, event: Event) {
        self.mouse.process_event(event);
    }
}