mod camera_plugin;
mod player_plugin;
mod terrain_plugin;

use bevy::prelude::*;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use camera_plugin::CameraPlugin;
use player_plugin::PlayerPlugin;
use terrain_plugin::TerrainPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((CameraPlugin, PlayerPlugin, TerrainPlugin))
        .add_systems(Startup, setup_instructions)
        .run();
}

fn setup_instructions(mut commands: Commands) {
    commands.spawn(Text::new("WASD per muoversi"));
}
