use bevy::{color::palettes::tailwind::BLUE_950, prelude::*};

use super::{GenericMechanicalComponentBundle, MyPosition, MyRigidBody, Shape};

#[derive(Component, Default)]
#[require(RobotLeg, RobotHead)]
pub struct Robot; // Tag per l'entità principale del robot.

#[derive(Component, Default)]
pub struct RobotHead;

#[derive(Component, Default)]
pub struct RobotLeg;

pub fn spawn_robot_head(
    command: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    command
        .spawn((
            RobotHead,
            GenericMechanicalComponentBundle::new(
                MyRigidBody::Dynamic { mass: 0.1 },
                Shape::Rect {
                    width: 50.,
                    heigt: 50.,
                },
                BLUE_950.into(),
                MyPosition { x: 0., y: 0. },
                meshes,
                materials,
            ),
        ))
        .id()
}

pub fn spawn_robot_leg(
    command: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    command
        .spawn((
            RobotLeg,
            GenericMechanicalComponentBundle::new(
                MyRigidBody::Dynamic { mass: 0.1 },
                Shape::Ball { radius: 30. },
                BLUE_950.into(),
                MyPosition { x: 0., y: -100. },
                meshes,
                materials,
            ),
        ))
        .id()
}
