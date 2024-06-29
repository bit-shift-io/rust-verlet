use std::{cell::RefCell, rc::Rc, time::Instant};

use cgmath::Vector2;
use sdl2::{event::Event, pixels::Color};

use crate::{application::{Context, Scene}, keyboard::Keyboard, mouse::Mouse, v2::{body::Body, solver::Solver}, v3::{particle_accelerator::ParticleAccelerator, particle_collider::ParticleCollider, particle_renderer::ParticleRenderer, shape_builder::ShapeBuilder, types::Vec2}};

use super::{car::Car, car_v2::CarV2, cloth::Cloth};

pub struct CarSceneContext<'a> {
    pub keyboard: &'a mut Keyboard,
    pub mouse: &'a mut Mouse,
    pub particle_accelerator: &'a mut ParticleAccelerator,
}

pub struct CarScene {
    // v2
    pub solver: Solver,
    pub car_v2: CarV2,
    pub keyboard: Keyboard,
    pub mouse: Mouse,

    // v3
    pub car: Car,
    pub cloth: Cloth,
    pub particle_accelerator: ParticleAccelerator,

    pub time: f32,
    pub update_instant: Instant,
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
        
        let cloth = Cloth::new(&mut particle_accelerator);

        // lets try a hanging particle on a spring
        ShapeBuilder::new()
            .set_spring_constant(20.0)
            .set_damping(5.0)
            .set_mass(1.0)
            .set_radius(10.0)
            .add_hanging_particle(Vec2::new(500.0, 100.0), Vec2::new(500.0, 200.0))
            .create_in_particle_accelerator(&mut particle_accelerator, mask);

        // add a jellow cube to the scene
        ShapeBuilder::new()
            .set_stiffness_factor(0.9)
            .set_mass(1.0)
            .set_radius(8.0)
            .add_stick_grid(2, 5, 20.0, Vec2::new(500.0, 680.0))
            .create_in_particle_accelerator(&mut particle_accelerator, mask);

        Self { 
            solver, 
            car_v2,
            keyboard: Keyboard::new(),
            mouse: Mouse::new(),
            particle_accelerator,
            car,
            cloth,
            time: 0.0,
            update_instant: Instant::now()
        }
    }
}

impl Scene for CarScene {
    fn update(&mut self, _context: &mut Context) {
        // compute how long the last frame took
        let last_elapsed = self.update_instant.elapsed();
        self.update_instant = Instant::now();
        //println!("Last update took: {} ms", last_elapsed.as_millis());

        self.keyboard.update();
        self.mouse.update();

        {
            let mut car_scene_context = CarSceneContext{ 
                keyboard: &mut self.keyboard, 
                mouse: &mut self.mouse,
                particle_accelerator: &mut self.particle_accelerator,
            };
            
            // v2
            self.car_v2.update(&mut car_scene_context);
            self.solver.update(0.0167f32);
        }

        // v3
        {
            // reset forces to just the gravity value
            // 9.8 = units are in metres per second
            // 980 = units are cm per second
            let gravity = Vec2::new(0.0, 9.8);
            let mut collider = ParticleCollider::new();
            collider.reset_forces(&mut self.particle_accelerator, gravity);

            // now update the car which will setup its forces on the particles
            let mut car_scene_context = CarSceneContext{ 
                keyboard: &mut self.keyboard, 
                mouse: &mut self.mouse,
                particle_accelerator: &mut self.particle_accelerator,
            };
            self.car.update(&mut car_scene_context);
            

            // finally, solve everything for this frame
            let desired_hertz = 100.0; // 100 times per second
            //let dt = last_elapsed.as_millis();
            //let dt = 0.0167f32; // 1 / 60
            //const SUB_STEPS: usize = 16; // higher substeps causes spring issues
            for sub_dt in collider.range_substeps_2(last_elapsed, desired_hertz).iter() {
                collider.solve_collisions(&mut self.particle_accelerator);
                collider.update_constraints(&mut self.particle_accelerator, *sub_dt);
                collider.update_positions(&mut self.particle_accelerator, *sub_dt);
            }
            //self.time += dt;

            let mut car_scene_context = CarSceneContext{ 
                keyboard: &mut self.keyboard, 
                mouse: &mut self.mouse,
                particle_accelerator: &mut self.particle_accelerator,
            };
            self.cloth.update(&mut car_scene_context, last_elapsed.as_secs_f32());

            //println!("time: {}", self.time);
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

    fn process_event(&mut self, _context: &mut Context, event: Event) {
        self.mouse.process_event(event.clone());
        self.keyboard.process_event(event.clone());
    }
}