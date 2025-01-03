pub mod robot_parts;

use bevy::{color::palettes::tailwind::BLUE_950, prelude::*};
use bevy_rapier2d::prelude::{GenericJoint, ImpulseJoint, RevoluteJoint, RevoluteJointBuilder};
use robot_parts::*;

use crate::mechanical_components::generic::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn spawn_robot(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    // Spawna l'entità principale del Robot
    let robot = commands
        .spawn((
            Robot,
            Transform::from_xyz(0., 100., 0.),
            Visibility::default(),
        ))
        .id();

    let head = spawn_robot_head(commands, meshes, materials);
    let leg = spawn_robot_leg(commands, meshes, materials);
    commands.entity(robot).add_child(head);

    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2 { x: 100., y: 100. })
        .local_anchor2(Vec2 { x: 0., y: 30. });
    commands.entity(leg).insert(ImpulseJoint::new(head, joint));
    robot
}
