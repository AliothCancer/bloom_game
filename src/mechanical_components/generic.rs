use bevy::{color::palettes::css::WHITE_SMOKE, math::VectorSpace, prelude::*};
use bevy_rapier2d::prelude::*;

const DEFAULT_COLOR: Srgba = WHITE_SMOKE;

#[derive(Bundle)]
pub struct GenericMechanicalComponentBundle {
    pub rigid_body: RigidBody,
    pub collider_mass: ColliderMassProperties,
    pub velocity: Velocity,
    pub ext_force: ExternalImpulse,
    pub damping: Damping,
    pub gravity_scale: GravityScale,
    pub position: Transform,
    pub mesh: Mesh2d,
    pub collider: Collider,
    pub material: MeshMaterial2d<ColorMaterial>,
}

pub struct MyPosition {
    pub x: f32,
    pub y: f32,
}
impl MyPosition{
    pub fn to_transform(&self) -> Transform{
        Transform::from_xyz(self.x, self.y, 0.0)
    }
}
pub enum Shape {
    Rect { width: f32, heigt: f32 },
    Ball { radius: f32 },
}
impl Shape {
    fn generate_mesh(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Mesh2d {
        match self {
            Shape::Rect { width, heigt } => Mesh2d(meshes.add(Rectangle::new(*width, *heigt))),
            Shape::Ball { radius } => Mesh2d(meshes.add(Circle::new(*radius))),
        }
    }
    fn generate_collider(&self) -> Collider {
        match self {
            Shape::Rect { width, heigt } => Collider::cuboid(width / 2., heigt / 2.),
            Shape::Ball { radius } => Collider::ball(*radius),
        }
    }
}
pub enum MyRigidBody {
    Dynamic { mass: f32 },
    Fixed,
}
impl GenericMechanicalComponentBundle {
    pub fn new(
        rigid_body: MyRigidBody,
        shape: Shape,
        color: Color,
        position: Transform,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        let (rigid_body, collider_mass) = match rigid_body {
            MyRigidBody::Dynamic { mass } => {
                (RigidBody::Dynamic, ColliderMassProperties::Mass(mass))
            }
            MyRigidBody::Fixed => (RigidBody::Fixed, ColliderMassProperties::Mass(0.0)),
        };

        Self {
            rigid_body,
            collider_mass,
            velocity: Velocity::default(),
            ext_force: ExternalImpulse{ impulse: Vec2::ZERO, torque_impulse: 0.0 },
            damping: Damping {
                linear_damping: 3.5,
                angular_damping: 0.4,
            },
            gravity_scale: GravityScale(0.0),
            position,
            material: MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
            mesh: shape.generate_mesh(meshes),
            collider: shape.generate_collider(),
        }
    }
}
