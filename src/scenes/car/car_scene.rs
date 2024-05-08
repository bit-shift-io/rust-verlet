use std::{cell::RefCell, rc::Rc};

use cgmath::{InnerSpace, Vector2};
use sdl2::{event::Event, gfx::primitives::DrawRenderer, keyboard::Keycode, pixels::Color};
use rand::Rng;

use crate::{application::{Context, Scene}, keyboard::Keyboard, mouse::Mouse, v2::{attachment::Attachment, body::Body, particle::Particle, solver::Solver, stick::Stick}, v3::{particle_accelerator::{ParticleAccelerator, ParticleCollider, ParticleRenderer}, shape_builder::ShapeBuilder, types::Vec2}};

use super::{car::Car, car_v2::CarV2};

pub struct CarSceneContext<'a> {
    pub keyboard: &'a mut Keyboard,
    pub mouse: &'a mut Mouse,
}

pub struct CarScene {
    // v2
    pub solver: Solver,
    pub car_v2: CarV2,
    pub keyboard: Keyboard,
    pub mouse: Mouse,

    // v3
    pub car: Car,
    pub particle_accelerator: ParticleAccelerator,
}

impl CarScene {
    pub fn new() -> Self {
        // v2
        let mut solver = Solver::new();

        let ground_plane = Rc::new(RefCell::new(Body::create_line(Vector2::new(100.0f32, 800.0f32), Vector2::new(600.0f32, 800.0f32), 8.0f32)));
        ground_plane.borrow_mut().set_static(true);
        solver.add_body(&ground_plane);

        let ground_plane_2 = Rc::new(RefCell::new(Body::create_line(Vector2::new(600.0f32, 800.0f32), Vector2::new(1000.0f32, 700.0f32), 8.0f32)));
        ground_plane_2.borrow_mut().set_static(true);
        solver.add_body(&ground_plane_2);

        let car_v2 = CarV2::new();
        car_v2.add_to_solver(&mut solver);

        // v3
        let mut particle_accelerator = ParticleAccelerator::new();

        let mask = 0x1;
        ShapeBuilder::new()
            .set_static(true)
            .add_line(Vec2::new(100.0f32, 800.0f32), Vec2::new(600.0f32, 800.0f32), 8.0f32)
            .add_line(Vec2::new(600.0f32, 800.0f32), Vec2::new(1000.0f32, 700.0f32), 8.0f32)
            .create_in_particle_accelerator(&mut particle_accelerator, mask);
        
        let car = Car::new(&mut particle_accelerator);

        Self { 
            solver, 
            car_v2,
            keyboard: Keyboard::new(),
            mouse: Mouse::new(),
            particle_accelerator,
            car
        }
    }
}

impl Scene for CarScene {
    fn update(&mut self, context: &mut Context) {
        self.keyboard.update();
        self.mouse.update();

        let mut car_scene_context = CarSceneContext{ keyboard: &mut self.keyboard, mouse: &mut self.mouse };
        
        // v2
        self.car_v2.update(&mut car_scene_context);
        self.solver.update(0.0167f32);

        // v3
        self.car.update(&mut car_scene_context);

        let mut collider = ParticleCollider::new();
        collider.reset_forces(&mut self.particle_accelerator);

        let dt = 0.0167f32;
        const SUB_STEPS: usize = 16;
        for sub_dt in collider.range_substeps(dt, SUB_STEPS).iter() {
            collider.solve_collisions(&mut self.particle_accelerator);
            collider.update_positions(&mut self.particle_accelerator, *sub_dt);
            collider.update_constraints(&mut self.particle_accelerator);
        }
    }

    fn draw(&mut self, context: &mut Context) {
        context.sdl.canvas.set_draw_color(Color::RGB(128, 255, 255));
        context.sdl.canvas.clear();

        // v2
        self.solver.draw(context.sdl);

        // v3
        let renderer = ParticleRenderer::new();
        renderer.draw(&mut context.sdl, &self.particle_accelerator);

        context.sdl.canvas.present();
    }

    fn process_event(&mut self, context: &mut Context, event: Event) {
        self.mouse.process_event(event.clone());
        self.keyboard.process_event(event.clone());
    }
}