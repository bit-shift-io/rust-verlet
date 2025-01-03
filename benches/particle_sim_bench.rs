#![feature(extract_if)]

use std::time::Duration;

use bevy::{color::{Color, LinearRgba}, math::vec2};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use v4::{particle::Particle, particle_sim::ParticleSim, particle_solvers::{naive_particle_solver::NaiveParticleSolver, spatial_hash_particle_solver::SpatialHashParticleSolver}, shape_builder::{circle::{self, Circle}, line_segment::LineSegment, rectangle::Rectangle, shape_builder::ShapeBuilder}};

#[path = "../src/v4/mod.rs"]
mod v4;

// TODO: split the setup from the benching of the update loop!
// I don't care to benchmark how long setup takes

// This lets us do some standard test on a solver to get some comparison
fn run_sim_solver_test(particle_sim: &mut ParticleSim) {
    // create some static shapes
    //{
        // static perimiter
        let mut perimeter = ShapeBuilder::new();
        perimeter.set_particle_template(Particle::default().set_static(true).set_radius(0.03).clone())
            .set_particle_template(Particle::default().set_static(true).clone())
            .apply_operation(circle::Circle::new(vec2(0.0, 0.0), 4.0))
            .create_in_particle_sim(particle_sim);

        // some dynamic particles on the inside
        let mut liquid = ShapeBuilder::new();
        liquid
            .set_particle_template(Particle::default().set_mass(20.0 * 0.001).set_radius(0.03).set_color(Color::from(LinearRgba::BLUE)).clone())
            .apply_operation(Rectangle::from_center_size(vec2(0.0, 0.0), vec2(3.0, 3.0)))
            .create_in_particle_sim(particle_sim);
    //}

    // step the simulation x second forward in time
    particle_sim.update(0.5);

    // print out the metrics to help us determine performance
    //println!("num_collision_checks: {}", sim.particle_solver.as_ref().get_metrics().num_collision_checks);
}

fn naive_particle_solver_particle_sim() {
    let particle_solver = Box::new(NaiveParticleSolver::new());
    let mut sim = ParticleSim::new(particle_solver);
    run_sim_solver_test(&mut sim);
}



fn spatial_hash_particle_solver_particle_sim() {
    let particle_solver = Box::new(SpatialHashParticleSolver::new());
    let mut sim = ParticleSim::new(particle_solver);
    run_sim_solver_test(&mut sim);
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    //c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));

    let mut group = c.benchmark_group("sample-size-example");
    group.sample_size(20);//.measurement_time(Duration::from_secs(10));

    group.bench_function("spatial_hash_particle_solver_particle_sim", |b| b.iter(|| spatial_hash_particle_solver_particle_sim()));
    group.bench_function("naive_particle_solver_particle_sim", |b| b.iter(|| naive_particle_solver_particle_sim()));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
