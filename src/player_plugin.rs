use bevy::{color::palettes::tailwind::RED_100, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::mechanical_component::generic::{
    GenericMechanicalComponentBundle, MyPosition, MyRigidBody, Shape,
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

#[derive(Component)]
struct Cube;
#[derive(Component)]
struct PlayerPart;

#[derive(Component)]
pub struct Player {
    pub lenght: f32,
    pub accel_force: f32,
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player {
            lenght: PLAYER_LENGTH,
            accel_force: PLAYER_ACCELERATION_FORCE,
        },
        GenericMechanicalComponentBundle::new(
            MyRigidBody::Dynamic { mass: 1. },
            Shape::Rect {
                width: PLAYER_LENGTH,
                heigt: PLAYER_LENGTH * 2.,
            },
            RED_100.into(),
            MyPosition { x: 0., y: 100. },
            &mut meshes,
            &mut materials,
        ),
    ));
}
const PLAYER_LENGTH: f32 = 50.; // meters
const PLAYER_ACCELERATION_FORCE: f32 = 1000.; // newton

fn move_player(
    ext_forces: Single<&mut ExternalForce, With<Player>>,
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
