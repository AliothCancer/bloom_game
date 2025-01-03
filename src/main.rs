#![allow(unused)]

mod camera_plugin;
mod mechanical_components;
mod robot_factory;
mod player_plugin;
mod terrain_plugin;

use bevy::{diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin}, prelude::*};
use bevy_rapier2d::prelude::*;
use camera_plugin::CameraPlugin;
use iyes_perf_ui::{
    entries::{PerfUiFramerateEntries, PerfUiSystemEntries, PerfUiWindowEntries},
    prelude::{PerfUiAllEntries, PerfUiEntryFPS, PerfUiEntryFPSWorst, PerfUiRoot},
    PerfUiPlugin,
};
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    text::FontSmoothing,
};
use player_plugin::PlayerPlugin;
use terrain_plugin::TerrainPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins((CameraPlugin, PlayerPlugin, TerrainPlugin))
        // debug
        .add_plugins(RapierDebugRenderPlugin::default())
        //.add_plugins((
        //    FrameTimeDiagnosticsPlugin,
        //    SystemInformationDiagnosticsPlugin,
        //    EntityCountDiagnosticsPlugin,
        //    PerfUiPlugin,
        //))
        //.add_systems(Update, toggle.before(iyes_perf_ui::PerfUiSet::Setup))
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 42.0,
                    font: default(),
                    font_smoothing: FontSmoothing::default(),
                },
                text_color: Color::srgb(0.0, 1.0, 0.0),
                enabled: true,
            },
        })
        // startup
        //.add_systems(Startup, setup_instructions)

        .run();
}
fn toggle(
    mut commands: Commands,
    q_root: Query<Entity, With<PerfUiRoot>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        if let Ok(e) = q_root.get_single() {
            // despawn the existing Perf UI
            commands.entity(e).despawn_recursive();
        } else {
            // create a simple Perf UI with default settings
            // and all entries provided by the crate:
            commands.spawn(PerfUiAllEntries::default());
        }
    }
}

fn setup_instructions(mut commands: Commands) {
    commands.spawn(Text::new("WASD per muoversi"));
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.read() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.read() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
