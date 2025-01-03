pub mod player_assembly;

use bevy::{
    color::palettes::tailwind::{BLUE_100, BLUE_950, RED_100},
    ecs::{observer::TriggerTargets, query::QueryData},
    prelude::*,
};
use bevy_rapier2d::prelude::*;

use crate::{
    mechanical_components::generic::{
        GenericMechanicalComponentBundle, MyPosition, MyRigidBody, Shape,
    },
    robot_factory::{
        robot_parts::{RobotBody, RobotHead},
        spawn_robot,
    },
};

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
    
    let player = commands
        .spawn((
            Player,
            Transform::from_xyz(0., 100., 0.),
            Visibility::default(),
        ))
        .id();

    // Head spawn
    let head = commands
        .spawn((
            RobotHead,
            GenericMechanicalComponentBundle::new(
                MyRigidBody::Dynamic { mass: 0.5 },
                Shape::Ball { radius: 60. },
                Color::linear_rgb(4., 0., 10.),
                MyPosition { x: 0., y: 100. },
                &mut meshes,
                &mut materials,
            ),
        ))
        .id();
    // build the joint
    let joint = RopeJointBuilder::new(60.)
        .local_anchor1(Vec2 { x: 60., y: 60. })
        .local_anchor2(Vec2 { x: 0., y: 0. });
    let body_part = commands
            .spawn((
                RobotBody,
                GenericMechanicalComponentBundle::new(
                    MyRigidBody::Dynamic { mass: 0.1 },
                    Shape::Ball { radius: 30. },
                    BLUE_950.into(),
                    MyPosition { x: 90. + 100., y: 100. },
                    &mut meshes,
                    &mut materials,
                ),
            ))
            .id();
    // attach the joint
    commands
        .entity(body_part)
        .insert(ImpulseJoint::new(head, joint));

    let mut robot = vec![head, body_part];
    for i in 1..=2 {
        let body_part = commands
            .spawn((
                RobotBody,
                GenericMechanicalComponentBundle::new(
                    MyRigidBody::Dynamic { mass: 0.1 },
                    Shape::Ball { radius: 10. * (3. - i as f32) },
                    BLUE_950.into(),
                    MyPosition { x: (200.*i as f32), y: 100. },
                    &mut meshes,
                    &mut materials,
                ),
            ))
            .id();
        robot.push(body_part);
    }
    for i in robot.windows(2){
        let part1 = i[0];
        let part2 = i[1];
        let joint = RopeJointBuilder::new(10.)
        .local_anchor1(Vec2 { x: 0., y: 30. })
        .local_anchor2(Vec2 { x: 0., y: -30. });
        commands.entity(part2).insert(ImpulseJoint::new(part1, joint));

    }

    // add child to player
    commands.entity(player).add_children(&robot);
}
const PLAYER_LENGTH: f32 = 50.; // meters
const PLAYER_ACCELERATION_FORCE: f32 = 1000.; // newton

fn move_player(
    ext_forces: Single<&mut ExternalForce, With<RobotHead>>,
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
