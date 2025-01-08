#![feature(extract_if)]
#![feature(portable_simd)]

use std::time::Duration;
use std::simd::f32x2;

use bevy::{color::{Color, LinearRgba}, math::{bounding::Aabb2d, vec2}};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use v5::{aabb_simd::AabbSimd, naive_particle_solver::NaiveParticleSolver, particle::Particle, particle_vec::SharedParticleVec, shape_builder::{circle::{self, Circle}, line_segment::LineSegment, rectangle::Rectangle, shape_builder::ShapeBuilder}, spatial_hash::SpatialHash, spatial_hash_particle_solver::SpatialHashParticleSolver, spatial_hash_simd::SpatialHashSimd, spatial_hash_simd_particle_solver::SpatialHashSimdParticleSolver};

#[path = "../src/v5/mod.rs"]
mod v5;

// TODO: split the setup from the benching of the update loop!
// I don't care to benchmark how long setup takes

// This lets us do some standard test on a solver to get some comparison
fn setup_sim_solver_test(shared_particle_vec: &mut SharedParticleVec) {
    let particle_radius = 1.0;

    // static
    let mut perimeter = ShapeBuilder::new();
    perimeter.set_particle_template(Particle::default().set_static(true).set_radius(particle_radius).clone())
        .apply_operation(circle::Circle::new(vec2(0.0, 0.0), 100.0))
        .create_in_shared_particle_vec(shared_particle_vec);

    let mut perimeter2 = ShapeBuilder::new();
    perimeter2.set_particle_template(Particle::default().set_static(true).set_radius(particle_radius).clone())
        .apply_operation(circle::Circle::new(vec2(0.0, 0.0), 100.0 + (particle_radius * 2.0)))
        .create_in_shared_particle_vec(shared_particle_vec);

    // some dynamic particles on the inside
    let mut liquid = ShapeBuilder::new();
    liquid
        .set_particle_template(Particle::default().set_mass(20.0 * 0.001).set_radius(particle_radius).set_color(Color::from(LinearRgba::BLUE)).clone())
        .apply_operation(Rectangle::from_center_size(vec2(0.0, 0.0), vec2(120.0, 120.0)))
        .create_in_shared_particle_vec(shared_particle_vec);
}

/*
fn run_sim_solver_test(particle_sim: &mut ParticleSim) {
    // step the simulation x second forward in time
    //particle_sim.update(0.5);
    particle_sim.update_step(0.01);
}
*/


fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("v5");
    //group.sample_size(20);//.measurement_time(Duration::from_secs(10));

    group.bench_function("update_positions", |b| {
        let mut solver = NaiveParticleSolver::default();
        let mut shared_particle_vec = SharedParticleVec::default();
        setup_sim_solver_test(&mut shared_particle_vec);
        solver.bind(&shared_particle_vec);
        solver.solve_collisions();

        b.iter(|| {    
            shared_particle_vec.as_ref().write().unwrap().update_positions(0.01);
        })
    });

    group.bench_function("NaiveParticleSolver solve_collisions", |b| {
        let mut solver = NaiveParticleSolver::default();
        let mut shared_particle_vec = SharedParticleVec::default();
        setup_sim_solver_test(&mut shared_particle_vec);
        solver.bind(&shared_particle_vec);

        b.iter(|| {
            solver.solve_collisions();
            //shared_particle_vec.as_ref().write().unwrap().update_positions(0.01);
        })
    });

    group.bench_function("SpatialHashParticleSolver solve_collisions", |b| {
        let mut solver = SpatialHashParticleSolver::default();
        let mut shared_particle_vec = SharedParticleVec::default();
        setup_sim_solver_test(&mut shared_particle_vec);
        solver.bind(&shared_particle_vec);

        b.iter(|| {
            solver.solve_collisions();
            //shared_particle_vec.as_ref().write().unwrap().update_positions(0.01);
        })
    });

    group.bench_function("SpatialHashSimdParticleSolver solve_collisions", |b| {
        let mut solver = SpatialHashSimdParticleSolver::default();
        let mut shared_particle_vec = SharedParticleVec::default();
        setup_sim_solver_test(&mut shared_particle_vec);
        solver.bind(&shared_particle_vec);

        b.iter(|| {
            solver.solve_collisions();
            //shared_particle_vec.as_ref().write().unwrap().update_positions(0.01);
        })
    });


    group.bench_function("SpatialHash insert_aabb + aabb_iter", |b| {

        b.iter(|| {
            const TILE_SIZE: usize = 1;
            
            let h = TILE_SIZE as f32 / 2.0;
            let e1 = 1;
            let e2 = 2;
            let mut db = SpatialHash::<usize>::new(); //default();
            db.insert_aabb(
                Aabb2d {
                    min: vec2(-h, -h),
                    max: vec2(h, h),
                },
                e1,
            );
            db.insert_aabb(
                Aabb2d {
                    min: vec2(h, h),
                    max: vec2(h, h),
                },
                e2,
            );
            let matches: Vec<usize> = db
                .aabb_iter(Aabb2d {
                    min: vec2(-h, -h),
                    max: vec2(h, h),
                })
                .collect();
            // assert_eq!(matches.len(), 2);
            assert!(matches.contains(&e1));
            assert!(matches.contains(&e2));
        })
    });


    group.bench_function("SpatialHashSimd insert_aabb + aabb_iter", |b| {

        b.iter(|| {
            const TILE_SIZE: usize = 1;
            
            let h = TILE_SIZE as f32 / 2.0;
            let e1 = 1;
            let e2 = 2;
            let mut db = SpatialHashSimd::<usize>::new(); //default();
            db.insert_aabb(
                &AabbSimd::from_min_max(
                    f32x2::from_array([-h, -h]),
                    f32x2::from_array([h, h]),
                ),
                e1,
            );
            db.insert_aabb(
                &AabbSimd::from_min_max(
                    f32x2::from_array([h, h]),
                    f32x2::from_array([h, h]),
                ),
                e2,
            );
            let matches: Vec<usize> = db
                .aabb_iter(&AabbSimd::from_min_max(
                    f32x2::from_array([-h, -h]),
                    f32x2::from_array([h, h]),
                ))
                .collect();
            // assert_eq!(matches.len(), 2);
            assert!(matches.contains(&e1));
            assert!(matches.contains(&e2));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
