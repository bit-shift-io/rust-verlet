use sdl2::{event::Event, pixels::Color};
use rand::Rng;

use crate::{application::{Context, Scene}, v3::{particle_collider::ParticleCollider, particle_renderer::ParticleRenderer}};

use crate::v3::{types::Vec2, particle_accelerator::ParticleAccelerator, shape_builder::ShapeBuilder};

pub struct RandomBodiesScene {
    // v3
    pub particle_accelerator: ParticleAccelerator,
}

impl RandomBodiesScene {
    pub fn new() -> Self {
        // v3
        let mut particle_accelerator = ParticleAccelerator::new();

        let mask = 0x1;
        ShapeBuilder::new()
            .set_static(true)
            .add_line(Vec2::new(100.0f32, 800.0f32), Vec2::new(600.0f32, 800.0f32), 8.0f32)
            .create_in_particle_accelerator(&mut particle_accelerator, mask);

        Self {
            particle_accelerator 
        }
    }
}

impl Scene for RandomBodiesScene {
    fn update(&mut self, context: &mut Context) {
        // v3
        let gravity = Vec2::new(0f32, 1000f32);
        let mut collider = ParticleCollider::new();
        collider.reset_forces(&mut self.particle_accelerator, gravity);

        let dt = 0.0167f32;
        const SUB_STEPS: usize = 16;
        for sub_dt in collider.range_substeps(dt, SUB_STEPS).iter() {
            collider.solve_collisions(&mut self.particle_accelerator);
            collider.update_positions(&mut self.particle_accelerator, *sub_dt);
            collider.update_constraints(&mut self.particle_accelerator, *sub_dt);
        }
    }

    fn draw(&mut self, context: &mut Context) {
        context.sdl.canvas.set_draw_color(Color::RGB(128, 255, 255));
        context.sdl.canvas.clear();

        // v3
        let renderer = ParticleRenderer::new();
        renderer.draw(&mut context.sdl, &self.particle_accelerator);

        context.sdl.canvas.present();
    }

    fn process_event(&mut self, context: &mut Context, event: Event) {
        match event {
            Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Left, x, y, .. } => {
                let xf = x as f32;
                let yf = y as f32;
                let mut rng = rand::thread_rng();

                let shape = 1;//rng.gen_range(0..=1);

                // single particle
                if shape == 0 {
                    let col = Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255));

                    // v3
                    let mask = 0x1;
                    self.particle_accelerator.create_particle(Vec2::new(xf, yf), 10f32, 1f32, mask, col);
                }

                // chain of 2 particles
                if shape == 1 {
                    let radius = 10f32;

                    // v3
                    let mask = 0x1;
                    ShapeBuilder::new()
                        .add_particle(Vec2::new(xf, yf), radius)
                        .add_particle(Vec2::new(xf + radius * 2.0, yf), radius)
                        .add_stick([-2, -1]) // -2 = second to last, -1 = last
                        .create_in_particle_accelerator(&mut self.particle_accelerator, mask);
        
                }
                /* 
                // chain of 3 circles
                if shape == 0 {
                    let radius = rng.gen_range(5..50) as f32;
                    let pos1 = Vector2::new(xf, yf);
                    let pos2 = Vector2::new(xf + radius, yf);
                    let pos3 = Vector2::new(xf - radius, yf);
                    let col = Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255));
                    let mass = radius;
                    let p1 = self.solver.add_particle(Particle::new(pos1, radius, mass, col));
                    let p2 = self.solver.add_particle(Particle::new(pos2, radius, mass, col));
                    let p3 = self.solver.add_particle(Particle::new(pos3, radius, mass, col));
                
                    let length = radius * 2f32;
                    self.solver.add_stick(Stick::new(length, p1, p2));
                    self.solver.add_stick(Stick::new(length, p1, p3));
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

                    let p1 = self.solver.add_particle(Particle::new(pos1, radius, mass, col));
                    let p2 = self.solver.add_particle(Particle::new(pos2, radius, mass, col));
                    let p3 = self.solver.add_particle(Particle::new(pos3, radius, mass, col));
                    let p4 = self.solver.add_particle(Particle::new(pos4, radius, mass, col));
                
                    //solver.add_stick(Stick::new((pos1 - pos2).magnitude(), p1, p2));
                    //solver.add_stick(Stick::new((pos2 - pos3).magnitude(), p2, p3));
                    //solver.add_stick(Stick::new((pos3 - pos4).magnitude(), p3, p4));
                    self.solver.add_stick(Stick::new((pos4 - pos1).magnitude(), p4, p1));


                    self.solver.add_stick(Stick::new((pos1 - pos3).magnitude(), p1, p3));
                    self.solver.add_stick(Stick::new((pos2 - pos4).magnitude(), p2, p4));
                }*/
            },
            _ => {}
        }
    }
}