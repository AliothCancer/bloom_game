#![allow(unused)]

mod camera_plugin;
mod mechanical_component;
mod player_plugin;
mod terrain_plugin;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_rapier2d::prelude::*;
use camera_plugin::CameraPlugin;
use iyes_perf_ui::{entries::{PerfUiFramerateEntries, PerfUiSystemEntries, PerfUiWindowEntries}, PerfUiPlugin};
use player_plugin::PlayerPlugin;
use terrain_plugin::TerrainPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((CameraPlugin, PlayerPlugin, TerrainPlugin))
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup_instructions)
        //.add_systems(Update, display_events)
        .run();
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
