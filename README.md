# rust-verlet


## v1

Just something that works.

## v2

Attempt to unify into a workable library of sorts.

## v3

Adding abstractions in order to support SIMD or GPU support if required.
Refer to: https://www.rustsim.org/blog/2020/03/23/simd-aosoa-in-nalgebra/

## Controls

~ = Change scene

### Cloth scene
Mouse wheel = change radius for cloth scene
Left mouse = pull
Right mouse = cut

## Architecture

src/scenes - contains a series of scenes which test the verlet particle systems
src/v1 - version 1 verlet systems
src/v2 - version 2 verlet systems

## Install

    run ```install.sh``` to setup rust project

    You need the vscode extensions:

        CodeLLDB
        rust-analyzer

## Development

### Manually

        cargo build
        cargo run

        cargo add XXX # to add a cargo package

### vscode

    ctrl + shift + b - to run the tasks

    open in vscode then just hit debug!

## Benchmark/Testing

    cargo bench
    cargo test

## Future

    * Switch to bevy math library: https://docs.rs/bevy/latest/bevy/math/ ?
    * Use Arena allocators for Particles, Sticks etc to get ride of Rc<RefCell> usage

## Tutorials & Links

    https://pikuma.com/blog/verlet-integration-2d-cloth-physics-simulation

    https://torlenor.org/rust/graphics/gamedev/2023/09/16/sdl2_with_rust.html
    https://github.com/wsandy1/rust-verlet-solver
    https://github.com/bit-shift-io/fluidic-space
    https://betterprogramming.pub/making-a-verlet-physics-engine-in-javascript-1dff066d7bc5

### Understanding Verlet

    https://www.youtube.com/watch?v=g55QvpAev0I&list=WL&index=63
    which explains its just the kinematic equation with time step forward and time step backwards.

    https://www.physicsclassroom.com/class/1dkin/Lesson-6/Kinematic-Equations
    
    
## Troubleshooting

Can't debug rust in vscode: https://stackoverflow.com/questions/77218022/why-is-my-debugger-in-vscode-not-working-with-rust-after-mac-update-to-sonoma-14
