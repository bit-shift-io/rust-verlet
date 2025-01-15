// https://github.com/IyesGames/iyes_perf_ui/blob/main/examples/custom_minimal.rs

use std::time::Duration;

use bevy::{app::{App, Startup, Update}, ecs::system::{lifetimeless::SRes, SystemParam}, prelude::{Commands, Component, Query, Res, ResMut, Resource}, time::Time};
use iyes_perf_ui::{entry::PerfUiEntry, prelude::{PerfUiEntryFPS, PerfUiRoot}, PerfUiAppExt, PerfUiPlugin};

use super::car_scene::CarScene;

pub fn performance_ui_build(app: &mut App) {

    // start of perf ui/metrics. we want Bevy to measure these values for us:
    app .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        //.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        //.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)

        .add_plugins(PerfUiPlugin);
        //.add_systems(Startup, setup_perf_ui);
        // end of perf ui/metrics

    app.add_perf_ui_simple_entry::<PerfUiTimeSinceLastClick>();
    app.init_resource::<PerfMetrics>();

    app.add_systems(Startup, performance_ui_setup);
    app.add_systems(Update, performance_ui_update);
}
/* 
fn setup_perf_ui(mut commands: Commands) {

    // create a simple Perf UI with default settings
    // and all entries provided by the crate:
    commands.spawn(PerfUiCompleteBundle::default());
}*/


fn performance_ui_update(
    time: Res<Time>,
    mut perf_metrics: ResMut<PerfMetrics>,
    mut query_car_scenes: Query<(&mut CarScene)>,
) {
    /*
    let mut car_scene = query_car_scenes.single_mut();
    let delta_seconds = time.delta_seconds();
    
    perf_metrics.last_update += delta_seconds;
    if (perf_metrics.last_update >= 1.0) {
        //println!("num_collision_checks_last_second {}", perf_metrics.num_collision_checks_last_second);
        perf_metrics.last_update -= 1.0;
        perf_metrics.num_collision_checks_last_second = 0;
    }

    perf_metrics.num_collision_checks_last_second += car_scene.particle_sim.particle_solver.get_metrics().num_collision_checks;
    */
}

fn performance_ui_setup(mut commands: Commands) {
    commands.spawn((
        PerfUiRoot::default(),
        PerfUiEntryFPS::default(),
        //PerfUiTimeSinceLastClick::default(),
    ));
}

/// Global resource to store the time when the mouse was last clicked
#[derive(Resource, Default)]
pub struct PerfMetrics {
    num_collision_checks_last_second: usize,
    last_update: f32
}

#[derive(Component, Default)]
pub struct PerfUiTimeSinceLastClick;

// Implement the trait for integration into the Perf UI
impl PerfUiEntry for PerfUiTimeSinceLastClick {
    type Value = usize;
    // Any system parameters we need in order to compute our value
    type SystemParam = (SRes<Time>, SRes<PerfMetrics>);

    // The text that will be shown as the Perf UI label
    fn label(&self) -> &str {
        "Num Collision Checks Last Second"
    }

    // We must return a sort key, to determine where to place the entry
    fn sort_key(&self) -> i32 {
        // We can hardcode a value here. A negative value will
        // make our entry appear always on top, before any default
        // entries with automatic sort keys.
        -1
    }

    fn update_value(
        &self,
        (time, perf_metrics): &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        let d = perf_metrics.num_collision_checks_last_second; //time.elapsed() - lastclick.last_click;
        Some(d)
    }

    // since we don't provide an implementation of `fn format_value`,
    // the value will just be printed with its `Debug` formatting.
}