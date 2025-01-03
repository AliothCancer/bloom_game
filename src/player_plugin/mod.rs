pub mod player_assembly;

use bevy::{color::palettes::tailwind::RED_100, ecs::{observer::TriggerTargets, query::QueryData}, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{mechanical_components::generic::{
    GenericMechanicalComponentBundle, MyPosition, MyRigidBody, Shape,
}, robot_factory::{robot_parts::RobotLeg, spawn_robot}};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

#[derive(Event)]
struct AttachJointEvent {
    player: Entity,
    parent: Entity,
}

#[derive(Component, QueryData)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player = commands.spawn((
        Player,
        Transform::from_xyz(0., 100., 0.),
        Visibility::default()
    )).id();

    // Inserting the Robot
    let robot = spawn_robot(&mut commands, &mut meshes, &mut materials);
    commands.entity(player).add_child(robot);
}
const PLAYER_LENGTH: f32 = 50.; // meters
const PLAYER_ACCELERATION_FORCE: f32 = 1000.; // newton



fn move_player(
    ext_forces: Single<&mut ExternalForce, With<RobotLeg>>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;
    let mut torque_rotation = 0f32;
    if kb_input.pressed(KeyCode::ArrowRight) {
        torque_rotation = -1.;
    }
    if kb_input.pressed(KeyCode::ArrowLeft) {
        torque_rotation = 1.;
    }
    if kb_input.pressed(KeyCode::KeyW) {
        direction += Vec2 { x: 0.0, y: 1. };
    }
    if kb_input.pressed(KeyCode::KeyS) {
        direction += Vec2 { x: 0.0, y: -1. };
    }
    if kb_input.pressed(KeyCode::KeyA) {
        direction += Vec2 { x: -1.0, y: 0. };
    }
    if kb_input.pressed(KeyCode::KeyD) {
        direction += Vec2 { x: 1.0, y: 0. };
    }
    if direction.x.abs() + direction.y.abs() > 1. {
        direction *= Vec2 {
            x: 1. / 2.0f32.sqrt(),
            y: 1. / 2.0f32.sqrt(),
        }
    }
    //  dbg!(**ext_forces);
    let mut ext_force = ext_forces.into_inner();
    ext_force.force = direction * PLAYER_ACCELERATION_FORCE;
    ext_force.torque = torque_rotation * PLAYER_ACCELERATION_FORCE * 100.;

}
