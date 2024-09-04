// https://github.com/IyesGames/iyes_perf_ui/blob/main/examples/custom_minimal.rs

use bevy::{app::{App, Startup}, ecs::system::{lifetimeless::SRes, SystemParam}, prelude::{Commands, Component}, time::Time};
use iyes_perf_ui::{entry::PerfUiEntry, prelude::{PerfUiEntryFPS, PerfUiRoot}, PerfUiAppExt, PerfUiPlugin};

pub fn performance_ui_build(app: &mut App) {

    // start of perf ui/metrics. we want Bevy to measure these values for us:
    app .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        //.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        //.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)

        .add_plugins(PerfUiPlugin);
        //.add_systems(Startup, setup_perf_ui);
        // end of perf ui/metrics

    app.add_perf_ui_simple_entry::<PerfUiTimeSinceLastClick>();
    app.add_systems(Startup, performance_ui_setup);
}
/* 
fn setup_perf_ui(mut commands: Commands) {

    // create a simple Perf UI with default settings
    // and all entries provided by the crate:
    commands.spawn(PerfUiCompleteBundle::default());
}*/

fn performance_ui_setup(mut commands: Commands) {
    commands.spawn((
        PerfUiRoot::default(),
        PerfUiEntryFPS::default(),
        PerfUiTimeSinceLastClick::default(),
    ));
    /* 
    commands.spawn((
        PerfUiTimeSinceLastClick::default(),
    ));*/
}

#[derive(Component, Default)]
pub struct PerfUiTimeSinceLastClick;

// Implement the trait for integration into the Perf UI
impl PerfUiEntry for PerfUiTimeSinceLastClick {
    type Value = u64;
    // Any system parameters we need in order to compute our value
    type SystemParam = (SRes<Time>); //, SRes<TimeSinceLastClick>);

    // The text that will be shown as the Perf UI label
    fn label(&self) -> &str {
        "Num Collisions"
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
        (time): &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        let d = 3; //time.elapsed() - lastclick.last_click;
        Some(d)
    }

    // since we don't provide an implementation of `fn format_value`,
    // the value will just be printed with its `Debug` formatting.
}