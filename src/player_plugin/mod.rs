pub mod player_assembly;

use bevy::{
    color::palettes::tailwind::{BLUE_100, BLUE_950, RED_100},
    ecs::{observer::TriggerTargets, query::QueryData},
    math::VectorSpace,
    prelude::*,
    text::cosmic_text::ttf_parser::head,
    transform,
    utils::hashbrown::HashMap,
};
use bevy_rapier2d::prelude::*;

use crate::{
    mechanical_components::generic::{
        GenericMechanicalComponentBundle, MyPosition, MyRigidBody, Shape,
    },
    robot_factory::{
        robot_parts::{Robot, RobotBody, RobotHead},
        spawn_robot,
    },
    MyTimer,
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
    // to keep track for measurments
    let mut robot_parts = vec![];
    let mut ball_radiuses = vec![];
    let mut positions = vec![];
    // player config
    let mut rope_distance = 250.;
    let player_pos = MyPosition { x: 0., y: 0. };

    // robot config
    let robot = Robot {
        rope_lenght: rope_distance,
    };
    let gap_between_balls = 9.;

    // head config
    let head_mass = 1.5;
    let head_color = Color::linear_rgb(3., 0., 16.);
    let head_radius = 100.;
    let head_pos = Transform::from_xyz(0.0, 0.0, 0.0);
    positions.push(head_pos);
    let loc_anchor1 = Vec2 { x: 0., y: 0. };
    let loc_anchor2 = Vec2 { x: 0., y: 0. };

    // body part 1 config
    let body_part1_radius = head_radius * 0.9;
    ball_radiuses.push(body_part1_radius);
    let body_part1_x = head_radius + body_part1_radius + gap_between_balls;
    positions.push(Transform::from_xyz(body_part1_x, 0., 0.));

    // other body part config
    let ball_nums = 48 * 3;
    let color_intensity = 10.;

    let player = commands
        .spawn((
            Player,
            TriggerOscillation(false),
            player_pos.to_transform(),
            robot,
            Visibility::default(),
        ))
        .id();

    // Head spawn
    let head = commands
        .spawn((
            RobotHead,
            GenericMechanicalComponentBundle::new(
                MyRigidBody::Dynamic { mass: head_mass },
                Shape::Ball {
                    radius: head_radius,
                },
                head_color,
                head_pos,
                &mut meshes,
                &mut materials,
            ),
        ))
        .id();
    robot_parts.push(head);
    // build the joint
    let head_joint = RopeJointBuilder::new(40.)
        .local_anchor1(loc_anchor1)
        .local_anchor2(loc_anchor2);
    let body_part1 = commands
        .spawn((
            RobotBody,
            GenericMechanicalComponentBundle::new(
                MyRigidBody::Dynamic {
                    mass: head_mass * 0.1,
                },
                Shape::Ball {
                    radius: body_part1_radius,
                },
                Color::linear_rgb(0., 0., 10.),
                Transform::from_xyz(body_part1_x, 0., 0.),
                &mut meshes,
                &mut materials,
            ),
        ))
        .id();
    robot_parts.push(body_part1);

    let mut impulse_joint = ImpulseJoint::new(head, head_joint);
    //if let TypedJoint::RopeJoint(mut rope_joint) = impulse_joint.data {
    //    rope_joint.set_contacts_enabled(false);
    //}
    // attach the joint
    commands.entity(body_part1).insert(impulse_joint);

    for i in 1..=ball_nums {
        let radius = head_radius * ((ball_nums - i + 1) as f32) / ball_nums as f32;
        let last_ball_radius = ball_radiuses.last().unwrap().to_owned();
        ball_radiuses.push(radius);
        let last_x_pos = positions.last().unwrap().translation.x;
        let x_pos = last_x_pos + radius + last_ball_radius + gap_between_balls;
        positions.push(Transform::from_xyz(x_pos, 0., 0.));
        let body_part = commands
            .spawn((
                RobotBody,
                GenericMechanicalComponentBundle::new(
                    MyRigidBody::Dynamic {
                        mass: head_mass * 0.1,
                    },
                    Shape::Ball { radius },
                    Color::linear_rgb(0., 0., (ball_nums - i) as f32 * color_intensity),
                    Transform::from_xyz(x_pos, 0., 0.),
                    &mut meshes,
                    &mut materials,
                ),
            ))
            .id();
        robot_parts.push(body_part);
    }
    for (n, pairs_of_parts) in robot_parts.windows(2).enumerate() {
        let part1 = pairs_of_parts[0];
        let part2 = pairs_of_parts[1];
        if n > 0 {
            rope_distance = ball_radiuses[n - 1] + ball_radiuses[n] + gap_between_balls + 3.;
        }
        let joint = RopeJointBuilder::new(rope_distance)
            .local_anchor1(Vec2 { x: 0., y: 0. })
            .local_anchor2(Vec2 { x: 0., y: 0. });
        let mut impulse_joint = ImpulseJoint::new(part1, joint);
        //if let TypedJoint::RopeJoint(mut rope_joint) = impulse_joint.data {
        //    rope_joint.set_contacts_enabled(false);
        //}
        commands.entity(part2).insert(impulse_joint);
    }

    // add child to player
    commands.entity(player).add_children(&robot_parts);
    //dbg!(positions);
}
const PLAYER_LENGTH: f32 = 50.; // meters
const PLAYER_ACCELERATION_FORCE: f32 = 50. * 9.; // newton

#[derive(Component)]
struct TriggerOscillation(bool);
fn move_player(
    mut oscillate: Single<&mut TriggerOscillation, With<Player>>,
    velocity: Single<(&mut ExternalImpulse, &mut Velocity), With<RobotHead>>,
    rope_length: Single<&mut Robot, With<Player>>,
    mut rope_entities: Query<&mut ImpulseJoint>,
    kb_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<MyTimer>,
) {
    let mut direction = Vec2::ZERO;
    let mut torque_rotation = 0f32;
    let mut rope_length = rope_length.into_inner();

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
        let module = direction.distance(Vec2::ZERO);
        direction /= module;
    }
    let dir_angle = direction.to_angle();
    let freq_mod = 6.;
    let oscillation = Vec2 {
        x: 0.0, //(timer.0.elapsed_secs() * freq_mod).cos(),
        y: (timer.0.elapsed_secs() * freq_mod).sin(),
    };

    let mut osc = oscillate.as_mut();
    if kb_input.pressed(KeyCode::KeyK) {
        osc.0 = !(osc.0);
    }
    if oscillate.into_inner().0 {
        direction += oscillation * 2.;
    }
    timer.0.tick(time.delta());

    //  dbg!(**ext_forces);
    let (mut impulse, mut velocity) = velocity.into_inner();
    //direction = velocity.linvel.normalize_or_zero().lerp(direction, 0.73);
    impulse.impulse += direction * PLAYER_ACCELERATION_FORCE * 200. * time.delta_secs();
}
